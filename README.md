# badge

[![Coverage Status](https://coveralls.io/repos/github/uzi0702/TEOTWAHBEW/badge.svg?branch=main)](https://coveralls.io/github/uzi0702/TEOTWAHBEW?branch=main)

# Tagline
TEOTWAHBEW is a CLI command like 'ls'

## Description

TEOTWAHBEW is a CLI command that displays files in the current directory.  
It highlights special files such as `Makefile` and `.gitignore` using distinct colors.

Colors can be configured using the `-C` option.

This software provides several options to display detailed information, such as file size.  
See the options below.

## Usage

```
teot [OPTION]
teot [--color=<color code>] [file]
```

## Options
```
-sd : Sort files by most recently updated

-c : Display only source code files

--color=(color code) [filename] : Choose display color for dsignated file
```

## Color codes

`--color` takes a numeric ANSI color code (not a color name like `red`).
Note that the `=` is required: `--color 31` is an error.

| Code | Color   |
|------|---------|
| 31   | red     |
| 32   | green   |
| 33   | yellow  |
| 34   | blue    |
| 35   | magenta |
| 36   | cyan    |
| 37   | white   |

Compound codes such as `34;1` (bold blue) are also accepted.

### Examples

```
teot --color=31              # show all files in red
teot --color=32 Cargo.toml   # show only Cargo.toml in green
teot --color='34;1' Makefile # show only Makefile in bold blue (quote the ';' for the shell)
```