#!/bin/sh

cdx() {
    case "$1" in
        -*)
            ~/.shx/bin/shx-cdx "$@"
            return
            ;;
    esac

    output="$(~/.shx/bin/shx-cdx "$@")"
    exit_code=$?

    if [ "$exit_code" -eq 0 ] && [ -n "$output" ]; then
        printf '\033[31m%s\033[0m\n' "$output"
        cd "$output" || return 1
    else
        printf 'cdx: failed to change directory\n' >&2
        printf 'err : %s\n' "$output" >&2
        return 1
    fi
}