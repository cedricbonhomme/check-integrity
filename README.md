


~~~~shell
$ echo 'export PATH="~/.cargo/bin:$PATH"' >> ~/.bashrc
$ cargo build --release
$ cargo install

$ check-integrity
check-integrity
Check integrity of files.

USAGE:
    check-integrity [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>          path to the top directory
    <expression>    regular expression



$ check-integrity ~ '**/*.pdf'
Looking for "/home/cedric/**/*.pdf"

 "/home/cedric/downloads/616.pdf"
   -> CRC32: 79F732FC
   -> CRC64: F636195053D3BC65

~~~~
