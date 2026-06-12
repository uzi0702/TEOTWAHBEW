---
title: "Examples"
weight: 3
---

# Examples

## List all files

```bash
$ teot
2024-01-15 12:00  4.0K  -rw-r--r--  Makefile
2024-01-14 09:30  1.2K  -rw-r--r--  .gitignore
2024-01-13 18:45  8.3K  -rw-r--r--  src/main.rs
```

## Sort by last updated

```bash
$ teot -sd
2024-01-15 12:00  4.0K  -rw-r--r--  Makefile
2024-01-14 09:30  1.2K  -rw-r--r--  .gitignore
2024-01-13 18:45  8.3K  -rw-r--r--  src/main.rs
```

## Show source files only

```bash
$ teot -c
2024-01-13 18:45  8.3K  -rw-r--r--  src/main.rs
```

## Apply a color to a file

```bash
$ teot --color=red Makefile
```

`Makefile` will be displayed in red.
