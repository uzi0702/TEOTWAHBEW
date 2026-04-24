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