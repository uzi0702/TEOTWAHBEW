---
title: "Options"
weight: 2
---

# Options

## -sd — Sort by Date

```bash
teot -sd
```

Sorts files by most recently updated (descending). If two files have the same timestamp, they are sorted alphabetically by name.

## -c — Source Files Only

```bash
teot -c
```

Displays only programming source files. The following extensions are included:

| Extension | Language    |
|-----------|-------------|
| `.rs`     | Rust        |
| `.py`     | Python      |
| `.c`      | C           |
| `.cpp`    | C++         |
| `.java`   | Java        |
| `.cs`     | C#          |
| `.js`     | JavaScript  |

## --color — Custom Color

```bash
teot --color=<colorcode> [file_name]
```

Sets the display color for a specific file. If `file_name` is omitted, the color is applied to all files.

```bash
# Color a specific file red
teot --color=red Makefile

# Color all files blue
teot --color=blue
```
