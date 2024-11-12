## CD eXtended
`cdx` is a simple extended `cd` command that allows you to change directory in a convenient way.

### Installation
1. Copy & paste the following function to your shell configuration file (e.g. `~/.bashrc`, `~/.zshrc`).

```shell
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
```

## Features
In case when your current directory is `foo/bar`, and wanna change directory to `foo/bar/baz`.. then,

### 0. Normal change directory as like `cd`.
- `cdx baz` works as same as `cd baz`.
- `cdx /foo/bar/baz` works as same as `cd foo/bar/baz`.

### 1. Change directory by end-matched.
> This feature requires you have visited the directory before.
- `cdx ^baz` works as `cd foo/bar/baz`
- `cdx ^bar/baz` works as `cd foo/bar/baz`

### 2. Change directory by revision.
> This feature requires you have visited the directory before.  
> You can see the latest history to check the revision by `cdx --show-history`

When you have a history like ...
```
2 /foo
1 /foo/bar
0 /foo/bar/baz
```

- `cdx ^1` works as `cd foo/bar/baz`
- `cdx ^2` works as `cd foo/bar`
- `cdx ^3` works as `cd foo`