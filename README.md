# copy1l

## Introduction

This small script sends the first line of stdin to the system clipboard.

For example, after running command:

```bash
{ echo foo; echo bar; echo baz; } | copy1l
```

the stdout will be:

```
foo
bar
baz
```

and `foo` will be sent to the system clipboard.

## Usage

Quoted from `copy1l --help`:

```
This small script sends the first line of stdin to the system clipboard.
All stdin inputs will be pushed as is to stdout

Usage: copy1l [OPTIONS]

Options:
  -q, --quote-single     Add single quotes around the line to copy before
                         copying. This is mutually exclusive with `-Qba`
  -Q, --quote-double     Add double quotes around the line to copy before
                         copying. This is mutually exclusive with `-qba`
  -b, --quote-backtick   Add backticks around the line to copy before copying.
                         This is mutually exclusive with `-qQa`
  -a, --quote-bash-auto  Add appropriate bash quotes around the line to copy
                         before copying. This is mutually exclusive with `-qQb`
  -n, --keep-newline     Don't trim the ending newline before copying
  -h, --help             Print help
  -V, --version          Print version
```

## Build and install

Clone this repo, and run:

```bash
cargo install --path .
```
