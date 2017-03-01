Bratwurst
---

[Brotli](https://github.com/google/brotli) based CLI compression tool.

This links directly against the Brotli library. This tool is a stream
compressor in the same spirit as `bzip2`, `gunzip`, `xz`, and many
more.

Its flags are therefore designed to be compatible with those tools as
I want to be able to use this with GNU-TAR (eventually).

This is a very early release, the `-f` and `-keep` flags aren't fully
supported (you'll always keep data, nothing will be deleted). More or
less it works pretty well for compressing large uncompressed tarballs.

###How to build?

```
1. Install rustup (to install cargo+rustc)
2. Install a local c-compiler
3. Clone this repo
4. cargo build --release
5. The executable is located in /target/release
```

###CLI Manpage

```
USAGE:
    bratwurst.exe [FLAGS] [OPTIONS]

FLAGS:
    -z, --compress
            Compress Data
    -d, --decompress
            Decompress data
    -f, --force
            Compatibility flag for interacting with GNU utils
    -h, --help          Prints help information
    -k, --keep
            Keep (don't delete) the input file
    -c, --stdout
            Compatibility flag for interacting with GNU utils
    -V, --version       Prints version information

OPTIONS:
    -b, --block <block>
            How large each block is [default: 128k]  [values: 64k, 128k, 256k, 512k, 1m, 2m, 4m]
    -i, --input <FILE>
            File to compress
    -l, --level <level>
            Compression Quality, speed vs ratio trade off [default: 3]  [values: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    -m, --mode <mode>
            Compression mode. WOFF and UTF8 text get special treatment [default: generic]  [values: generic, text, font]
    -o, --output <FILE>
            File that we compressed
    -t, --threads <NUM>
            When compressing do so with multiple threads
    -w, --window <window>
            Sliding window to find matches on [default: 8k]  [values: 1k, 2k, 4k, 8k, 16k, 32k, 64k, 128k, 256k, 512k,
            1m, 2m, 4m]

```


