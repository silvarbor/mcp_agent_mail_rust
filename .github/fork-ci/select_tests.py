#!/usr/bin/env python3
"""Select the tests that exercise code the fork changed.

Prints a cargo-nextest filter expression for the tests that live in Rust files
the fork changed relative to an upstream commit, minus the tests listed as
failing on that upstream commit itself.

A test is worth running in the fork's CI when a failure would be the fork's to
fix. A test in a file the fork never touched can only fail for upstream's
reasons or the runner's, so it is left to the full-suite job.

Usage:
    select_tests.py --base <upstream-commit> [--exclude <file>] [--explain]

The repository must be the current directory, and both the base commit and
HEAD must be present in it.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from collections import defaultdict

MOD_DECL = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*(;|\{)"
)
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
TARGET_KINDS = {"lib", "bin", "test", "proc-macro"}


def run(*cmd: str) -> str:
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout


def strip_noise(line: str) -> str:
    """Drop string literals and line comments so braces can be counted."""
    line = re.sub(r'"(?:\\.|[^"\\])*"', '""', line)
    line = re.sub(r"'(?:\\.|[^'\\])'", "''", line)
    return line.split("//", 1)[0]


def declared_modules(path: str):
    """Yield (module segments below this file, declared file path or None).

    Follows `mod name;` declarations, `#[path = "..."]` attributes and
    declarations nested in inline `mod name { ... }` blocks.
    """
    try:
        with open(path, encoding="utf-8", errors="replace") as handle:
            lines = handle.readlines()
    except OSError:
        return
    inline: list[tuple[str, int]] = []
    depth = 0
    pending_path = None
    for raw in lines:
        attr = PATH_ATTR.search(raw)
        if attr:
            pending_path = attr.group(1)
        line = strip_noise(raw)
        match = MOD_DECL.match(line)
        if match:
            name, kind = match.groups()
            if kind == ";":
                yield [seg for seg, _ in inline] + [name], pending_path
            else:
                inline.append((name, depth))
            pending_path = None
        elif line.strip() and not line.lstrip().startswith("#"):
            pending_path = None
        depth += line.count("{") - line.count("}")
        while inline and depth <= inline[-1][1]:
            inline.pop()


def module_tree(root_file: str) -> dict[str, str]:
    """Map every file of a target to the module path it defines."""
    files: dict[str, str] = {}

    def visit(path: str, module: list[str], is_dir_owner: bool) -> None:
        path = os.path.normpath(path)
        if path in files or not os.path.isfile(path):
            return
        files[path] = "::".join(module)
        here = os.path.dirname(path)
        own_dir = (
            here
            if is_dir_owner
            else os.path.join(here, os.path.splitext(os.path.basename(path))[0])
        )
        for segments, explicit in declared_modules(path):
            base = os.path.join(own_dir, *segments[:-1])
            name = segments[-1]
            if explicit:
                target = os.path.join(here if len(segments) == 1 else base, explicit)
                visit(target, module + segments, os.path.basename(target) == "mod.rs")
                continue
            flat = os.path.join(base, name + ".rs")
            nested = os.path.join(base, name, "mod.rs")
            if os.path.isfile(flat):
                visit(flat, module + segments, False)
            else:
                visit(nested, module + segments, True)

    visit(root_file, [], True)
    return files


def targets() -> list[tuple[str, str]]:
    """Return (nextest binary id, root source file) for each test-bearing target."""
    meta = json.loads(
        run("cargo", "metadata", "--no-deps", "--format-version", "1", "--locked")
    )
    root = meta["workspace_root"]
    found = []
    for package in meta["packages"]:
        for target in package["targets"]:
            kinds = set(target["kind"])
            if not kinds & TARGET_KINDS:
                continue
            source = os.path.relpath(target["src_path"], root)
            if kinds & {"lib", "proc-macro"}:
                binary = package["name"]
            elif "bin" in kinds:
                binary = f"{package['name']}::bin/{target['name']}"
            else:
                binary = f"{package['name']}::{target['name']}"
            found.append((binary, source))
    return found


def changed_files(base: str) -> list[str]:
    return [
        line
        for line in run("git", "diff", "--name-only", base, "HEAD").splitlines()
        if line
    ]


def referencing_sources(data_files: list[str], sources: list[str]) -> set[str]:
    """Rust files that name a changed data file count as changed themselves."""
    hits: set[str] = set()
    if not data_files:
        return hits
    texts = {}
    for source in sources:
        try:
            with open(source, encoding="utf-8", errors="replace") as handle:
                texts[source] = handle.read()
        except OSError:
            continue
    for data in data_files:
        base = os.path.basename(data)
        parent = os.path.basename(os.path.dirname(data))
        named = {source for source, text in texts.items() if base in text}
        if not named and parent:
            named = {
                source
                for source, text in texts.items()
                if f"{parent}/" in text or f'"{parent}"' in text
            }
        hits |= named
    return hits


def test_prefix(module: str) -> str:
    return "^" + re.escape(module).replace("\\:", ":") + "::"


def build_filter(base: str, exclude_file: str | None, explain: bool) -> str:
    changed = changed_files(base)
    if any(
        name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml") for name in changed
    ):
        selection = "all()"
        if explain:
            print(
                "workspace manifest, lockfile or toolchain changed: selecting every test",
                file=sys.stderr,
            )
    else:
        trees = [(binary, module_tree(source)) for binary, source in targets()]
        all_sources = sorted({path for _, tree in trees for path in tree})
        changed_rust = {name for name in changed if name.endswith(".rs")}
        data = [
            name
            for name in changed
            if not name.endswith(
                (".rs", ".md", ".sh", ".yml", ".yaml", ".toml", ".lock")
            )
        ]
        changed_rust |= referencing_sources(data, all_sources)
        terms = []
        for binary, tree in trees:
            modules = set(tree.values())
            for path, module in sorted(tree.items()):
                if path not in changed_rust:
                    continue
                children = sorted(
                    other
                    for other in modules
                    if other != module
                    and (not module or other.startswith(module + "::"))
                    and not any(
                        mid != module
                        and mid != other
                        and other.startswith(mid + "::")
                        and (not module or mid.startswith(module + "::"))
                        for mid in modules
                    )
                )
                term = f"binary_id(={binary})"
                if module:
                    term += f" & test(/{test_prefix(module)}/)"
                if children:
                    term += (
                        " & not ("
                        + " | ".join(
                            f"test(/{test_prefix(child)}/)" for child in children
                        )
                        + ")"
                    )
                terms.append(f"({term})")
                if explain:
                    print(f"{path} -> {binary} {module or '<root>'}", file=sys.stderr)
        selection = "(" + " | ".join(terms) + ")" if terms else "none()"
        if explain:
            unmapped = sorted(changed_rust - set(all_sources))
            for name in unmapped:
                print(f"{name} -> no workspace target", file=sys.stderr)

    excluded: dict[str, list[str]] = defaultdict(list)
    if exclude_file and os.path.isfile(exclude_file):
        with open(exclude_file, encoding="utf-8") as handle:
            for line in handle:
                line = line.strip()
                if not line or line.startswith("#"):
                    continue
                binary, _, name = line.partition(" ")
                excluded[binary].append(name.strip())
    if excluded:
        parts = [
            f"(binary_id(={binary}) & ("
            + " | ".join(f"test(={name})" for name in sorted(names))
            + "))"
            for binary, names in sorted(excluded.items())
        ]
        selection += " & not (" + " | ".join(parts) + ")"
    return selection


FINAL_STATUS = re.compile(
    r"^\s+(?:FAIL|TIMEOUT|SIG[A-Z]+|TRY \d+ [A-Z]+)\s+\[[^\]]*\]\s+(?:\([^)]*\)\s+)?(\S+)\s+(\S+)"
)
ANSI = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")


def failed_tests(log_path: str) -> list[tuple[str, str]]:
    """Read the failed tests from the lines nextest prints after its summary."""
    with open(log_path, encoding="utf-8", errors="replace") as handle:
        lines = [ANSI.sub("", line) for line in handle]
    summaries = [index for index, line in enumerate(lines) if "Summary [" in line]
    if not summaries:
        return []
    failed = set()
    for line in lines[summaries[-1] :]:
        match = FINAL_STATUS.match(line)
        if match and not match.group(1).startswith("("):
            failed.add((match.group(1), match.group(2)))
    return sorted(failed)


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--base", help="upstream commit the fork is built on")
    parser.add_argument(
        "--list-failures",
        metavar="LOG",
        help="print the failed tests of a nextest log, in the --exclude file format",
    )
    parser.add_argument(
        "--exclude", help="file listing tests that fail on the upstream commit"
    )
    parser.add_argument(
        "--explain",
        action="store_true",
        help="print the file-to-module mapping on stderr",
    )
    args = parser.parse_args()
    if args.list_failures:
        for binary, name in failed_tests(args.list_failures):
            print(binary, name)
        return 0
    if not args.base:
        parser.error("--base is required unless --list-failures is given")
    print(build_filter(args.base, args.exclude, args.explain))
    return 0


if __name__ == "__main__":
    sys.exit(main())
