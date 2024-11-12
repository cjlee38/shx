#!/bin/sh

cdx() {
    case "$1" in
        -*)
            printf "got option: %s\n" "$1" >&2
            ~/.shx/bin/shx-cdx "$@"
            return
            ;;
    esac

    dir_path="$(~/.shx/bin/shx-cdx "$@")"
    exit_code=$?

    if [ "$exit_code" -eq 0 ] && [ -n "$dir_path" ]; then
        printf '\033[31m%s\033[0m\n' "$dir_path"
        cd "$dir_path" || return 1
    else
        printf 'cdx: failed to change directory\n' >&2
        printf 'err : %s\n' "$dir_path" >&2
        return 1
    fi
}