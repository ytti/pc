# pc

pc (paste-client) is a command line tool for uploading text to a pastebin
server. It supports many different pastebin style servers, and is configurable.
It aims to be simple, work with stdin/stdout, and adhere to the unix
philosophy.


## Features

- [ ] supported servers
  - [X] generic (paste.rs)
  - [X] hastebin
  - [X] vpaste
  - [X] fiche (termbin)
  - [X] fedora pastebin
  - [ ] pastebin.com
- [X] configuration file for providing defaults and server configurations
- [X] quickly list configured servers, backends, full config, detailed backend
  information
- [X] comprehensive graceful error handling
- [X] hardcoded sensible defaults for use without config file
- [X] paste url history
- [ ] optional arguments for servers that support:
  - [ ] expire time
  - [ ] public/private
  - [ ] filetype
  - [ ] password protected
  - [ ] title

TODO: work out nice api for showing which server backends support which
arguments

## Usage examples

```
$ echo "Hello" | pc
https://paste.rs/saC

$ pc --help
<usage instructions>

$ pc vpaste < code.txt
http://vpaste.net/example

$ pc fedora --title "foo debug log" < debug.log
https://paste.fedoraproject.org/paste/7Taaazf88VimfqOnriOsFg

$ pc list
rs => generic | https://paste.rs/ [default]
vpaste => generic | http://vpaste.net/
haste => haste | https://hastebin.com/

$ pc list-backends
generic
haste
vpaste

$ pc show-backend generic
The generic backend works for any pastebin service that accepts the data in the
body of a POST request and returns the access url in plain text in the response
body.

Example:

  [servers.rs]
  backend = "generic"
  url = "https://paste.rs/"

$ pc dump-config
<toml config as currently used>

$ pc -c NONE dump-config
<default config as toml>

$ echo "hi" | pc --histfile NONE
http://vpaste.net/example
```

## Configuration

Configuration is via a toml file. The configuration file is determined by the
following:

1. file given to the `-c` command line arg
  a) if filename is `NONE`, skip to 5
  b) if file isn't found, exit with error
2. otherwise use `$XDG_CONFIG_HOME/pc/config.toml` if exists
3. otherwise use `$HOME/.config/pc/config.toml` if exists
5. finally, no files found; use hardcoded defaults (see what hardcoded defaults
   are with `pc -c NONE dump-config`


See [default_config.toml](./default_config.toml) for an example config file.
(This is also baked into the app as the default config.)

## Supported servers

| server spec                                             | backend      | example instance                 |
| ------                                                  | -------      | ---------------                  |
| [vpaste](http://pileus.org/tools/vpaste)                | vpaste       | http://vpaste.net/               |
| [Haste](https://github.com/seejohnrun/haste-server)     | haste        | https://hastebin.com/            |
| [paste.rs](https://paste.rs/web)                        | generic      | https://paste.rs/                |
| [fiche](https://github.com/solusipse/fiche)             | fiche        | https://termbin.com/             |
| [modern_paste](https://github.com/LINKIWI/modern-paste) | modern_paste | https://paste.fedoraproject.org/ |

See the [wiki page](https://github.com/swalladge/pc/wiki/server-list) for a list of public server instances supported.

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
