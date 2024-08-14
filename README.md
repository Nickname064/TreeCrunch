# TreeCrunch
A simple tool to turn a heap of files into a single one.
IMPORTANT: This is not compression, and is rather written so as to be able to embed entire filesystems into C applications.

## How
Provide `TreeCrunch` with one or multiple paths, which it will treat as being in the root directory.
It will then turn all of those files into a single one, with the following structure
- [Header size(u64)]
- [Headers] -> { [file size(u64)], [file start index(u64)], [file path(null-teminated CString)] }
- [File(s)]

## Notes
This is meant to be used in parallel with C-embed, another tool.
This way, you can convert a filesystem into a single file, then embed it into a C application.
Please look at the provided reader in `reader.c`.
If, for some reason, you don't wan't to embed the crunched file, but still want to use it, just `mmap` it
