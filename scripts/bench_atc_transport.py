"""Measure ATC overhead through persistent MCP servers with equal mailboxes."""

import argparse
import hashlib
import json
import math
import os
import selectors
import sqlite3
import statistics
import subprocess
import sys
import time
from contextlib import closing
from pathlib import Path

WARMUP_SENDS = 3
MEASURED_SENDS = 50
SEED_FORWARD_SENDS = 50
SEED_REVERSE_SENDS = 10
RPC_TIMEOUT_SECONDS = 30
SHUTDOWN_TIMEOUT_SECONDS = 20
SUBJECT = "ATC transport benchmark"
MODES = {
    "off": "mail_send_no_atc",
    "shadow": "mail_send_atc_shadow",
    "live": "mail_send_atc_live",
}


class Rpc:
    def __init__(self, process: subprocess.Popen[bytes]) -> None:
        if process.stdin is None or process.stdout is None:
            raise RuntimeError("stdio pipes are required")
        self.stdin = process.stdin
        self.stdout = process.stdout
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.stdout, selectors.EVENT_READ)
        self.buffer = b""
        self.request_id = 0

    def notify(self, method: str) -> None:
        self.stdin.write(
            json.dumps({"jsonrpc": "2.0", "method": method}).encode() + b"\n"
        )
        self.stdin.flush()

    def call(self, method: str, params: dict[str, object]) -> dict[str, object]:
        self.request_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params,
        }
        self.stdin.write(json.dumps(request).encode() + b"\n")
        self.stdin.flush()
        deadline = time.monotonic() + RPC_TIMEOUT_SECONDS
        while True:
            while b"\n" in self.buffer:
                line, self.buffer = self.buffer.split(b"\n", 1)
                response = json.loads(line)
                if not isinstance(response, dict) or response.get("jsonrpc") != "2.0":
                    raise RuntimeError("invalid JSON-RPC response")
                if "id" not in response and "method" in response:
                    continue
                if response.get("id") != self.request_id:
                    raise RuntimeError(f"unexpected response id: {response.get('id')}")
                result = response.get("result")
                if (
                    "error" in response
                    or not isinstance(result, dict)
                    or result.get("isError")
                ):
                    raise RuntimeError(f"request {self.request_id} failed: {response}")
                return result
            remaining = deadline - time.monotonic()
            if remaining <= 0 or not self.selector.select(remaining):
                raise TimeoutError(f"request {self.request_id} timed out")
            chunk = os.read(self.stdout.fileno(), 65536)
            if not chunk:
                raise RuntimeError("server closed stdout before responding")
            self.buffer += chunk

    def tool(self, name: str, arguments: dict[str, object]) -> dict[str, object]:
        return self.call("tools/call", {"name": name, "arguments": arguments})

    def close(self, process: subprocess.Popen[bytes]) -> None:
        self.selector.close()
        try:
            self.stdin.close()
        except BrokenPipeError:
            # A failed peer can close its pipe before teardown; still reap it.
            pass
        try:
            process.wait(timeout=SHUTDOWN_TIMEOUT_SECONDS)
        except subprocess.TimeoutExpired:
            process.terminate()
            try:
                process.wait(timeout=SHUTDOWN_TIMEOUT_SECONDS)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            raise RuntimeError("server did not shut down after EOF") from None
        finally:
            self.stdout.close()
        if process.returncode != 0:
            raise RuntimeError(f"server exited {process.returncode}")


def measure(binary: Path, root: Path, mode: str) -> dict[str, object]:
    root.mkdir()
    home = root / "home"
    home.mkdir()
    database = root / "mailbox.sqlite3"
    environment = {
        key: os.environ[key] for key in ("PATH", "TMPDIR") if key in os.environ
    }
    environment.update(
        {
            "HOME": str(home),
            "XDG_CONFIG_HOME": str(home / "config"),
            "XDG_CACHE_HOME": str(home / "cache"),
            "XDG_DATA_HOME": str(home / "data"),
            "DATABASE_URL": f"sqlite:///{database}",
            "STORAGE_ROOT": str(root / "archive"),
            "AM_ALLOW_EPHEMERAL_PROJECT_ROOTS": "1",
            "TUI_ENABLED": "false",
            "RUST_LOG": "error",
            "AM_ATC_ENABLED": "true",
            "ATC_LEARNING_DISABLED": "1" if mode == "off" else "0",
            "AM_ATC_WRITE_MODE": mode,
        }
    )
    samples: list[float] = []
    seed_count = SEED_FORWARD_SENDS + SEED_REVERSE_SENDS
    total_sends = seed_count + WARMUP_SENDS + MEASURED_SENDS
    with (root / "server.log").open("wb") as stderr:
        process = subprocess.Popen(
            [str(binary), "serve-stdio"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=stderr,
            env=environment,
            cwd=root,
        )
        rpc = Rpc(process)
        try:
            rpc.call(
                "initialize",
                {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "atc-performance-gate", "version": "1"},
                },
            )
            rpc.notify("notifications/initialized")
            project = str(root / "project")
            rpc.tool("ensure_project", {"human_key": project})
            for agent in ("BlueLake", "RedFox"):
                rpc.tool(
                    "register_agent",
                    {
                        "project_key": project,
                        "name": agent,
                        "program": "benchmark",
                        "model": "benchmark",
                    },
                )
            for index in range(total_sends):
                reverse = SEED_FORWARD_SENDS <= index < seed_count
                started = time.perf_counter_ns()
                rpc.tool(
                    "send_message",
                    {
                        "project_key": project,
                        "sender_name": "RedFox" if reverse else "BlueLake",
                        "to": ["BlueLake" if reverse else "RedFox"],
                        "subject": SUBJECT,
                        "body_md": "benchmark",
                    },
                )
                elapsed_ms = (time.perf_counter_ns() - started) / 1_000_000
                if index >= seed_count + WARMUP_SENDS:
                    samples.append(elapsed_ms)
        finally:
            rpc.close(process)
    with closing(sqlite3.connect(f"file:{database}?mode=ro", uri=True)) as connection:
        integrity = connection.execute("PRAGMA integrity_check").fetchall()
        sent = connection.execute(
            "SELECT COUNT(*) FROM messages WHERE subject = ?", (SUBJECT,)
        ).fetchone()[0]
    if integrity != [("ok",)] or sent != total_sends:
        raise RuntimeError(
            f"{mode}: mailbox integrity/count mismatch: {integrity}, {sent}"
        )
    atc_path = database.with_name("atc.sqlite3")
    atc_rows = 0
    if atc_path.is_file():
        with closing(
            sqlite3.connect(f"file:{atc_path}?mode=ro", uri=True)
        ) as connection:
            atc_integrity = connection.execute("PRAGMA integrity_check").fetchall()
            atc_rows = connection.execute(
                "SELECT COUNT(*) FROM atc_experiences"
            ).fetchone()[0]
        if atc_integrity != [("ok",)]:
            raise RuntimeError(f"{mode}: ATC integrity failed: {atc_integrity}")
    if atc_rows != (total_sends if mode == "live" else 0):
        raise RuntimeError(f"{mode}: unexpected durable ATC row count: {atc_rows}")
    ordered = sorted(samples)
    return {
        "median_ms": statistics.median(samples),
        "mean_ms": statistics.mean(samples),
        "p95_ms": ordered[math.ceil(len(ordered) * 0.95) - 1],
        "p99_ms": ordered[math.ceil(len(ordered) * 0.99) - 1],
        "samples_ms": samples,
        "messages": sent,
        "atc_rows": atc_rows,
        "integrity": "ok",
        "store": str(root),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--store-root", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with binary.open("rb") as executable:
        binary_sha256 = hashlib.file_digest(executable, "sha256").hexdigest()
    root = args.store_root.resolve()
    root.mkdir(parents=True, exist_ok=False)
    benchmarks: dict[str, object] = {}
    failures: list[dict[str, str]] = []
    for mode, name in MODES.items():
        try:
            benchmarks[name] = measure(binary, root / mode, mode)
        except (OSError, RuntimeError, ValueError, sqlite3.Error) as error:
            failures.append({"benchmark": name, "error": str(error)})
            break
    print(
        json.dumps(
            {
                "measurement": {
                    "binary": str(binary),
                    "binary_sha256": binary_sha256,
                    "transport": "persistent_mcp_stdio",
                    "stores": "independent_equal_seed",
                    "warmup": WARMUP_SENDS,
                    "runs": MEASURED_SENDS,
                    "absolute_baseline_comparable": False,
                    "baseline_note": "Historical CLI process timings are not comparable to persistent MCP request timings.",
                },
                "summary": {"benchmarks": benchmarks},
                "failures": failures,
            },
            indent=2,
        )
    )
    return int(bool(failures))


if __name__ == "__main__":
    sys.exit(main())
