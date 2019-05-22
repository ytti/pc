# PC

PC (paste-client) is a command line tool for uploading text to a pastebin
server. It supports many different pastebin style servers, and is configurable.
It aims to be simple, work with stdin/stdout, and adhere to the unix
philosophy.


## Usage examples

```
$ echo "Hello" | pc
https://paste.rs/saC

$ pc --help
<usage instructions>

$ pc --server vpaste < code.txt
https://paste.rs/saC

$ pc --list-servers
rs: (generic) https://paste.rs/
vpaste: (generic) http://vpaste.net/
haste: (hastebin) https://hastebin.com/

$ pc --list-backends
generic
hastebin

$ pc --backend-info generic
The generic backend works for any pastebin service that accepts the data in the
body of a POST request and returns the access url in plain text in the response
body.

Example:

  [servers.vpaste]
  backend = "generic"
  url = "http://vpaste.net/"

$ pc --print-config
<toml config as currently used>
```


## Configuration

Configuration is via a toml file. The configuration file is searched for in the
following locations, and the first one found use used:

1. file given to the `-c` command line arg. (exits with error if this file
   isn't found)
2. `$XDG_CONFIG_HOME/pc/config.toml`
3. `$HOME/.config/pc/config.toml`
4. `$HOME/.pc.toml`
5. no files; hardcoded defaults are used


Example config file:

```toml
[main]
default = "rs"

[servers]

[servers.vpaste]
backend = "generic"
url = "http://vpaste.net/"

[servers.rs]
# generic backend is also default
url = "https://paste.rs/"

[servers.haste]
backend = "hastebin"
url = "https://hastebin.com/"
```


## License

Copyright © 2019 Samuel Walladge

Dual licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
