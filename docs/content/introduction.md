---
title: "Introduction"
weight: 1
---

# TEOTWAHBEW

**TEOTWAHBEW** (`teot`) is a Rust-based CLI tool that lists files in the current directory — similar to `ls`, but with colored highlights for special files such as `Makefile` and `.gitignore`.

## Installation

Download the latest binary from [GitHub Releases](https://github.com/uzi0702/TEOTWAHBEW/releases) and place it in your `PATH`.

```bash
# Example (macOS / Linux)
chmod +x teot
mv teot /usr/local/bin/
```

## Basic Usage

```bash
teot
```

Running `teot` without any arguments lists all files and directories in the current directory, displaying:

- Last updated time
- Size (human-readable)
- File mode
