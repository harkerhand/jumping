# Jumping

> **J**umping **U**nlocks **M**ulti-path **P**recise **I**nstant **N**avigating **G**ear.

A lightweight TUI tool for lightning-fast directory teleportation.

# Use

add to `.bashrc` or `.zshrc`

```shell
jp() {
[ -f /tmp/jumping_result ] && rm /tmp/jumping_result

    jumping

    if [ -f /tmp/jumping_result ]; then
        local DEST=$(cat /tmp/jumping_result)
        if [ -d "$DEST" ]; then
            cd "$DEST"
            pwd
        fi
        rm /tmp/jumping_result
    fi
}
```
