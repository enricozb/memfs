# In-Memory Filesystem

## Overview
This repo contains three crates:
- `fs`: filesystem types with no facilities
- `session`: operations on a filesystem
- `repl`: an interactive read-eval-print-loop tui to demonstrate `session` functionality

## Documentation
Run `cargo doc --open` while at the root of any one of the three crates to see their documentation.

## Repl

### Overview
Run `cargo run -r -p repl` while at the top level to interact with an ephemeral filesystem.
For example:
```sh
cargo run -r -p repl

"/" >>> mkdir foo
"/" >>> touch foo/bar.txt
"/" >>> fill foo/bar.txt
"/" >>> mkdir baz
"/" >>> mv foo/bar.txt baz/bar.txt
"/" >>> cat baz/bar.txt
csMETNqOeb...
"/" >>> ls
d "baz"
d "foo"
"/" >>> tree
· "/"
  · "baz"
    · "bar.txt"
  · "foo"
"/" >>>
```

### Details
The repl has a `help` command that describes the available commands. Additionally,
`help <command>` will give additional information on commands.
```
"/" >>> help
Usage: A in-memory filesystem repl. Use ctrl-c to exit.

Commands:
  stat   Display file or directory metadata
  cd     Change directory
  ls     List directory entries
  mkdir  Create a new directory
  touch  Creates an empty file
  fill   Fills a file with random data
  cat    Prints a file's content
  rm     Remove a directory or file
  mv     Move a file or directory. The destination will be the source's new name, as opposed to the source's new parent. This will overwrite the destination if one exists
  tree   List contents of directories in a tree-like format
  find   List all file paths under the current directory with the given name
  help   Print this message or the help of the given subcommand(s)
```
