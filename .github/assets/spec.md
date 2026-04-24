# Input / Output

Input:
    teot [OPTIONS]
    or
    teot [--color=<colorcode>] [file_type]

Output:
    Prints files and directories in the current directory to stdout,
    including the following information:
        - Last updated time
        - Size (human-readable bytes)
        - File mode
        - and more

# OPTIONS

-sd :
    Sort files by most recently updated.
    If timestamps are the same, sort by name.

-c :
    Display only programming source files.
    Target extensions: .rs .py .c .cpp .java .cs .js

--color=<colorcode> [file_name] :
    Set display color for the specified file.
    If no file is specified, apply the color to all files.