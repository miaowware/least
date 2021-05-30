# least

**Support/chat:** [![Discord server](https://discordapp.com/api/guilds/656888365886734340/widget.png?style=shield)](https://discord.gg/SwyjdDN)
[![irc.libera.chat channel #miaowware](https://www.miaow.io/irc_shield.svg)](https://web.libera.chat/?channel=#miaowware)

[![crates.io - least version](https://img.shields.io/crates/v/least.svg?logo=rust)](https://crates.io/crates/least)
[![crates.io - least downloads](https://img.shields.io/crates/d/least.svg?logo=rust)](https://crates.io/crates/least)
[![crates.io - least license](https://img.shields.io/crates/l/least.svg)](https://crates.io/crates/least)

A simple terminal pager, written in Rust.  
⚠️ **Work in progress! Currently not fully usable as a replacement for `less`.** ⚠️

## Usage

```sh
# you can pipe program output into least
$ some_program_output | least
# or read from a file
$ least some/text/file.ext
```

```
$ least --help
...
USAGE:
    least [FLAGS] [FILE]

FLAGS:
        --no-page    Don't page output. Useful for passing text through least.
                     Default if stdout is not a TTY.
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    File to display
```

## Future features

- Configuration file
- Configurable keybinds
- Line wrapping
- Status bar
- Search
- Line numbers
- Syntax highlighting
- Colour codes passthrough

## Copyright

Copyright © 2021 classabbyamp, 0x5c  
Released under the BSD 3-Clause License.  
See [`LICENSE`](LICENSE) for the full license text.
