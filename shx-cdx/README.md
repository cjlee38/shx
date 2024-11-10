## CD eXtended
`cdx` is a simple extended `cd` command that allows you to change directory in a convenient way.

### Installation
1. Copy & paste the following function to your shell configuration file (e.g. `~/.bashrc`, `~/.zshrc`).

```shell
cdx() {
    new_dir=$(/Users/cjlee/Desktop/workspace/shx/shx-cdx/target/debug/cdx "$@")
    if [ $? -eq 0 ]; then
                builtin cd $new_dir
        echo "success $new_dir"
    else
        echo "fail $new_dir"
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

### 2. Change directory by history number.
> This feature requires you have visited the directory before.  
> See history to check the number by `cdx --show-history`

When you have a history like ...
```
2 /foo
1 /foo/bar
0 /foo/bar/baz
```

- `cdx ^0` works as `cd foo/bar/baz`
- `cdx ^1` works as `cd foo/bar`
- `cdx ^2` works as `cd foo`