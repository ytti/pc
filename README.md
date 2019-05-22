# PC

PC (paste-client) is a command line tool for uploading text to a pastebin
server. It supports many different pastebin style servers, and is configurable.
It aims to be simple, work with stdin/stdout, and adhere to the unix
philosophy.

NOTE: this is under early development. Not all the features listed in the
readme are implemented yet.


## Usage examples

```
$ echo "Hello" | pc
https://paste.rs/saC

$ pc --help
<usage instructions>

$ pc --server vpaste < code.txt
http://vpaste.net/example

$ pc --list-servers
rs: (generic) https://paste.rs/
vpaste: (generic) http://vpaste.net/
haste: (haste) https://hastebin.com/

$ pc --list-backends
generic
haste
vpaste

$ pc --backend-info generic
The generic backend works for any pastebin service that accepts the data in the
body of a POST request and returns the access url in plain text in the response
body.

Example:

  [servers.rs]
  backend = "generic"
  url = "https://paste.rs/"

$ pc --print-config
<toml config as currently used>
```


## Configuration

Configuration is via a toml file. The configuration file is determined by the
following:

1. file given to the `-c` command line arg. (exits with error if this file
   isn't found)
2. check `$XDG_CONFIG_HOME/pc/config.toml`
3. check `$HOME/.config/pc/config.toml`
5. no files found; hardcoded defaults used


Example config file:

```toml
[main]
# optional; if missing, will use random server entry
default = "rs"

[servers]

# must be at least one server defined
[servers.vpaste]
backend = "vpaste"
url = "http://vpaste.net/"

[servers.rs]
backend = "generic"
url = "https://paste.rs/"

[servers.haste]
backend = "haste"
url = "https://hastebin.com/"
```

## Supported servers

| server                                              | backend                | example url           |
| ------                                              | -------                | ---------------       |
| [vpaste](http://pileus.org/tools/vpaste)            | vpaste (unimplemented) | http://vpaste.net/    |
| [Haste](https://github.com/seejohnrun/haste-server) | haste                  | https://hastebin.com/ |
| [paste.rs](https://paste.rs/web)                    | generic                | https://paste.rs/     |


## Development

Standard cargo project. `cargo build`, `cargo run`, et al.

There is a Makefile for some other common tasks. Eg. `make fmt` will run
rustfmt on all source files.


## License

Copyright © 2019 Samuel Walladge

Dual licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
