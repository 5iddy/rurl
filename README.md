# rurl - a curl clone written in rust

## Setup instructions

This program has only been tested with stable rust and requires cargo to installed. This program has been tested on Windows and Linux Operating systems. Let me know how it works on MacOS.

1. `git clone https://github.com/5iddy/rurl`
2. `cd rurl`
3. `cargo build --release`
4. Optional copy the `target/release/rurl` to a folder included in your PATH environment.

## Usage Instructions
Try running `rurl --help`

```shell
rurl 0.1.0
A modern alternative to curl built in rust

USAGE:
    rurl [OPTIONS] <URL>

ARGS:
    <URL>    Url to send the request to

OPTIONS:
    -b, --cookies <COOKIES>
            load cookies with -b <key>=<value>

    -c, --save-cookies-to <SAVE_COOKIES_TO>
            save cookies to <filename>

    -d, --data <DATA>
            Add body to the request

        --download
            

    -h, --help
            Print help information

    -H, --headers <HEADERS>
            Add headers with -H, you can add multiple headers

    -o, --out <filename>
            Output response to file with -o <filename>

    -q <QUERY_PARAMS>
            Add query parameters using -q

    -V, --version
            Print version information

    -X, --method <METHOD>
            Http methods to use: GET, POST, PUT, DELETE, HEAD, OPTIONS
```