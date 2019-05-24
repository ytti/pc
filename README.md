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

$ pc --server vpaste < code.txt
http://vpaste.net/example

$ pc list-servers
rs => generic | https://paste.rs/
vpaste => generic | http://vpaste.net/
haste => haste | https://hastebin.com/

$ pc list-backends
generic
haste
vpaste

$ pc backend-info generic
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


Example config file:

```toml
[main]
# optional; if missing, will use random server entry
server = "rs"

# optional; if missing, will not log url history.
# if set, will append url of successful pastes to this file, one per line,
# newest to the bottom. The file is created if not existing. Parent directories
# will not be created. If paste successful but failed to write histfile, a
# warning will be printed to stderr and will exit with a non-zero exit code.
histfile = "/tmp/paste_history.txt"

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

[servers.termbin]
backend = "fiche"
domain = "termbin.com"
# port is optional; defaults to 9999
port = 9999

[servers.fedora]
backend = "modern_paste"
url = "https://paste.fedoraproject.org/"
```

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
