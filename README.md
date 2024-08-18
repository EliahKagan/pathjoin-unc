# pathjoin-unc - Joining Windows paths, some with leading `\\`

These are some results of joining paths and path fragments, including some unusual (or invalid) paths whose semantics are not obvious, on Windows using the Rust standard library.

## Output

On Windows 10:

```text
\\?\  +  \\?\  =  \\?\
\\?\  +  \\.\  =  \\.\
\\?\  +  \\?   =  \\?\\?
\\?\  +  \\.   =  \\?\\
\\?\  +  C:    =  C:
\\?\  +  C:/   =  C:/
\\?\  +  C:\   =  C:\
\\.\  +  \\?\  =  \\?\
\\.\  +  \\.\  =  \\.\
\\.\  +  \\?   =  \\.\\\?
\\.\  +  \\.   =  \\.\\\.
\\.\  +  C:    =  C:
\\.\  +  C:/   =  C:/
\\.\  +  C:\   =  C:\
\\?   +  \\?\  =  \\?\
\\?   +  \\.\  =  \\.\
\\?   +  \\?   =  \\?
\\?   +  \\.   =  \\.
\\?   +  C:    =  C:
\\?   +  C:/   =  C:/
\\?   +  C:\   =  C:\
\\.   +  \\?\  =  \\?\
\\.   +  \\.\  =  \\.\
\\.   +  \\?   =  \\?
\\.   +  \\.   =  \\.
\\.   +  C:    =  C:
\\.   +  C:/   =  C:/
\\.   +  C:\   =  C:\
C:    +  \\?\  =  \\?\
C:    +  \\.\  =  \\.\
C:    +  \\?   =  C:\\?
C:    +  \\.   =  C:\\.
C:    +  C:    =  C:
C:    +  C:/   =  C:/
C:    +  C:\   =  C:\
C:/   +  \\?\  =  \\?\
C:/   +  \\.\  =  \\.\
C:/   +  \\?   =  C:\\?
C:/   +  \\.   =  C:\\.
C:/   +  C:    =  C:
C:/   +  C:/   =  C:/
C:/   +  C:\   =  C:\
C:\   +  \\?\  =  \\?\
C:\   +  \\.\  =  \\.\
C:\   +  \\?   =  C:\\?
C:\   +  \\.   =  C:\\.
C:\   +  C:    =  C:
C:\   +  C:/   =  C:/
C:\   +  C:\   =  C:\
```

## License

[0BSD](LICENSE)
