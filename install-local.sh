#!/usr/bin/env bash
# install-local.sh — Build from source and install to ~/.local/bin (or $DEST).
#
# Builds in a private directory under Cargo's resolved target directory.
# Installs the freshly-built binaries, then removes the private build output.
#
# Usage:
#   ./install-local.sh              # build release, install to ~/.local/bin
#   DEST=/usr/local/bin ./install-local.sh   # custom destination
#   ./install-local.sh --debug      # install debug build instead
#
# Fixes: https://github.com/Dicklesworthstone/mcp_agent_mail_rust/issues/79

set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$REPO_ROOT"

DEST="${DEST:-$HOME/.local/bin}"
PROFILE="release"
PROFILE_FLAG="--release"

for arg in "$@"; do
  case "$arg" in
    --debug)
      PROFILE="debug"
      PROFILE_FLAG=""
      ;;
    --help|-h)
      echo "Usage: $0 [--debug]"
      echo ""
      echo "Build from source and install am + mcp-agent-mail to ~/.local/bin."
      echo "Set DEST to override the install directory."
      echo "Temporary build artifacts are removed on exit."
      echo ""
      echo "Options:"
      echo "  --debug    Install debug build instead of release"
      exit 0
      ;;
    *)
      echo "ERROR: Unknown option: $arg. Run $0 --help." >&2
      exit 2
      ;;
  esac
done

BIN_CLI="am"
BIN_SERVER="mcp-agent-mail"

# macOS clears inherited DYLD variables when starting the system shell.
# Restore the compiler's library path for its LLVM stripping helper.
if [ "$(uname -s)" = "Darwin" ]; then
  rust_sysroot=$("${RUSTC:-rustc}" --print sysroot)
  if [ -f "$rust_sysroot/lib/libLLVM.dylib" ]; then
    export DYLD_LIBRARY_PATH="$rust_sysroot/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"
  fi
fi

# Resolve the Cargo target directory the same way Cargo does.
# This respects CARGO_TARGET_DIR, .cargo/config.toml [build] target-dir, etc.
get_target_dir() {
  if command -v cargo >/dev/null 2>&1; then
    local meta
    meta=$(cargo metadata --no-deps --format-version 1 2>/dev/null)
    if command -v jq >/dev/null 2>&1; then
      echo "$meta" | jq -r '.target_directory'
    else
      # Fallback: sed-based JSON extraction (works when path has no quotes)
      echo "$meta" | sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p'
    fi
  fi
}

TARGET_DIR=$(get_target_dir)
if [ -z "$TARGET_DIR" ]; then
  # Fallback: check CARGO_TARGET_DIR, then default ./target
  TARGET_DIR="${CARGO_TARGET_DIR:-$(pwd)/target}"
fi

mkdir -p "$TARGET_DIR"
BUILD_DIR=$(mktemp -d "$TARGET_DIR/install-local.XXXXXXXX")
# Cargo's cleanup guard requires this marker even if compiler startup fails.
printf '%s\n' 'Signature: 8a477f597d28d172789f06886806bc55' > "$BUILD_DIR/CACHEDIR.TAG"
export CARGO_TARGET_DIR="$BUILD_DIR"
export CARGO_BUILD_BUILD_DIR="$BUILD_DIR"
export CARGO_INCREMENTAL=0

cleanup() {
  local status=$?
  trap - EXIT
  if ! cargo clean --target-dir "$BUILD_DIR"; then
    echo "ERROR: Could not clean temporary build directory: $BUILD_DIR" >&2
    status=1
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

echo "==> Building $PROFILE artifacts..."
# shellcheck disable=SC2086
cargo build --locked $PROFILE_FLAG -p mcp-agent-mail -p mcp-agent-mail-cli \
  --bin "$BIN_CLI" --bin "$BIN_SERVER"

CLI_SRC="$BUILD_DIR/$PROFILE/$BIN_CLI"
SERVER_SRC="$BUILD_DIR/$PROFILE/$BIN_SERVER"

# Verify the artifacts exist and are executable
for bin in "$CLI_SRC" "$SERVER_SRC"; do
  if [ ! -x "$bin" ]; then
    echo "ERROR: Expected binary not found: $bin" >&2
    echo "" >&2
    echo "The temporary Cargo target directory is: $BUILD_DIR" >&2
    exit 1
  fi
done

# Show what we built
CLI_VERSION=$("$CLI_SRC" --version)
echo "==> Built: $CLI_SRC ($CLI_VERSION)"

# Create destination if needed
mkdir -p "$DEST"

# Atomic install: write to temp, then rename (prevents partial copies)
for bin in "$BIN_CLI" "$BIN_SERVER"; do
  src="$BUILD_DIR/$PROFILE/$bin"
  dst="$DEST/$bin"
  tmp="${dst}.tmp.$$"
  install -m 0755 "$src" "$tmp"
  mv -f "$tmp" "$dst"
done

INSTALLED_VERSION=$("$DEST/$BIN_CLI" --version)
echo "==> Installed to $DEST"
echo "    $DEST/$BIN_CLI ($INSTALLED_VERSION)"
echo "    $DEST/$BIN_SERVER"

# Verify both installed files before discarding the build output.
for bin in "$BIN_CLI" "$BIN_SERVER"; do
  if ! cmp -s "$BUILD_DIR/$PROFILE/$bin" "$DEST/$bin"; then
    echo "ERROR: Installed binary differs from the build: $DEST/$bin" >&2
    exit 1
  fi
done

# Check PATH
case ":$PATH:" in
  *":$DEST:"*) ;;
  *)
    echo ""
    echo "NOTE: $DEST is not in your PATH."
    echo "  Add it:  export PATH=\"$DEST:\$PATH\""
    ;;
esac
