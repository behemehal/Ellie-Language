#!/usr/bin/env bash
set -euo pipefail

TOOLS_DIR="$(cd "$(dirname "$0")" && pwd)"
GEN2=0

usage() {
    echo "Usage: ./run-test.sh <test-path> [--gen2]"
    echo "  test-path  path relative to tests/, without extension"
    echo "  --gen2     use gen2 compiler and VM"
    echo ""
    echo "Examples:"
    echo "  ./run-test.sh math/fib"
    echo "  ./run-test.sh gen2/hello --gen2"
}

if [ $# -eq 0 ]; then
    usage
    exit 1
fi

TEST="$1"
shift

for arg in "$@"; do
    case "$arg" in
        --gen2) GEN2=1 ;;
        *) echo "Unknown option: $arg"; usage; exit 1 ;;
    esac
done

TEST_FILE="tests/${TEST}.ei"

if [ ! -f "$TOOLS_DIR/$TEST_FILE" ]; then
    echo "ERROR: test file not found: $TOOLS_DIR/$TEST_FILE"
    exit 1
fi

cd "$TOOLS_DIR"

if [ "$GEN2" -eq 1 ]; then
    OUTPUT="tests/${TEST}.eic2"

    echo ""
    echo "==> Compiling $TEST_FILE (gen2)"
    cargo run --manifest-path=../elliec/Cargo.toml -- compile "$TEST_FILE" -e -o byteCode2 -p "$OUTPUT" --disable-colors

    echo ""
    echo "==> Running $OUTPUT (gen2)"
    cargo run --manifest-path=../ellievm/Cargo.toml -- run "$OUTPUT" -a
else
    OUTPUT="tests/${TEST}"
    DEBUG_FILE="tests/${TEST}.eig"

    echo ""
    echo "==> Compiling $TEST_FILE"
    cargo run --manifest-path=../elliec/Cargo.toml -- compile "$TEST_FILE" -s -e -o byteCode -p "$OUTPUT" --disable-colors

    echo ""
    echo "==> Running $OUTPUT"
    cargo run --manifest-path=../ellievm/Cargo.toml -- run "$OUTPUT" -d "$DEBUG_FILE" -a
fi

echo ""
echo "==> Done"
