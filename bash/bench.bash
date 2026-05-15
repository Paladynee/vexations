#!/usr/bin/env bash

component="${1:-lexer}"
shift || true

extra=""
if [[ "${1:-}" == "native" ]]; then
    extra="-C target-cpu=native"
    shift || true
elif [[ "$component" == "native" ]]; then
    component="lexer"
    extra="-C target-cpu=native"
    # native already shifted out by component
fi

# while we have test -v in bash 4.2, lets support older versions too.
# "${RUSTFLAGS+set}" expands to "set" if RUSTFLAGS is currently set, or "" if it
# isnt
if [ -z "${RUSTFLAGS+set}" ]; then
    # RUSTFLAGS unset
    RUSTFLAGS="$extra"
elif [ -z "$RUSTFLAGS" ]; then
    # RUSTFLAGS set but empty!
    RUSTFLAGS=""
else
    # RUSTFLAGS set and nonempty
    RUSTFLAGS="$RUSTFLAGS $extra"
fi

case "$component" in
    lexer)
        cargo bench -p vexations-compiler --bench lexer "$@"
        ;;
    parser)
        cargo bench -p vexations-compiler --bench parser "$@"
        ;;
    *)
        echo "Unknown component: $component"
        echo "Usage: $0 [lexer|parser] [native] [cargo-bench-args...]"
        exit 1
        ;;
esac
