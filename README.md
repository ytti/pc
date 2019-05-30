# pc

pc (paste-client) is a command line tool for uploading text to a pastebin
server. It supports many different pastebin style servers, and is highly
configurable.  It aims to be simple, work with stdin/stdout, and adhere to the
unix philosophy.


## Features

- Many supported servers. If your favourite pastebin isn't supported, please
  open an issue.
- Configuration file for providing defaults and server configurations.
- Comprehensive command line help. Quickly list configured servers, backends,
  full config, detailed backend information.
- Comprehensive graceful error handling.
- Baked in, sane defaults for use without config file.
- Optional paste url history.
- Optional arguments for servers that support extra features, such as title,
  filetype, private pastes, expire time, etc.


## Usage examples

Simplest, out of the box usage:

```
$ echo "Hello" | pc
https://paste.rs/saC
```

Select a custom server:

```
$ pc vpaste < code.txt
http://vpaste.net/example
```

Each configured server accepts cli args to override defaults, depending on
which backend is used. Here, the `fedora` server block uses the `modern_paste`
backend, which allows setting a custom title for the paste.
Note: can only use server-specific args if specifying the server on the cli.

```
$ pc fedora --title "foo debug log" < debug.log
https://paste.fedoraproject.org/paste/7Taaazf88VimfqOnriOsFg
```

To see a server block's configuration, backend, and allowed args:

```
$ pc fedora --help
Config for this server block:

ModernPaste(
    Backend {
        url: "https://paste.fedoraproject.org/",
        title: None
    }
)

modern_paste backend

USAGE:
    fedora [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --title <title>    Title for the paste
    -u, --url <url>        Url
```

Show a concise list of configured servers available to use:

```
$ pc list
rs => generic | https://paste.rs/ [default]
vpaste => generic | http://vpaste.net/
haste => haste | https://hastebin.com/
```

List all supported backends:

```
$ pc list-backends
generic
haste
vpaste
...
```

Show info and configuration help for a particular backend:

```
$ pc show-backend generic
The generic backend works for any pastebin service that accepts the data in the
body of a POST request and returns the access url in plain text in the response
body.

Example:

  [servers.rs]
  backend = "generic"
  url = "https://paste.rs/"
```

Dump the current config as interpreted. Helpful for debugging.

```
$ pc dump-config
<toml config as currently used>
```

Copy the default config to the user config file. Useful for first setup.

```
$ pc -c NONE dump-config > ~/.config/pc/config.toml
<default config as toml>
```

Histfile feature can also be disabled/enabled on the cli:

```
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
5. finally, no files found; use defaults (see what defaults
   are with `pc -c NONE dump-config` or see the default config file in this
   repo.

See [default_config.toml](./default_config.toml) for an example config file.
(This is also baked into the app as the default config.)


## Supported server backends

| server spec                                                       | backend         | example instance                 |
| ------                                                            | -------         | ---------------                  |
| [Haste](https://github.com/seejohnrun/haste-server)               | `haste`         | https://hastebin.com/            |
| [dpaste.com](http://dpaste.com/api/v2/)                           | `dpaste_com`    | http://dpaste.com/               |
| [fiche](https://github.com/solusipse/fiche)                       | `fiche`         | https://termbin.com/             |
| [Modern Paste](https://github.com/LINKIWI/modern-paste)           | `modern_paste`  | https://paste.fedoraproject.org/ |
| [ONE-TIME SECRET](https://github.com/onetimesecret/onetimesecret) | `onetimesecret` | https://onetimesecret.com/       |
| [paste.rs](https://paste.rs/web)                                  | `generic`       | https://paste.rs/                |
| [vpaste](http://pileus.org/tools/vpaste)                          | `vpaste`        | http://vpaste.net/               |

See the [wiki page](https://github.com/swalladge/pc/wiki/server-list) for a list of public server instances supported.


## Development

Standard cargo project. `cargo build`, `cargo run`, et al.

There is a Makefile for some other common tasks. Eg. `make fmt` will run
rustfmt on all source files.


## Related projects

- [pastebinit](https://launchpad.net/pastebinit): similar concept, supports
  config file, several pastebin servers, and options like private paste, set
  title, etc. An advantage is that this is available in several distros'
  official repos.
- curl: most lightweight pastebins support uploading text in a curl-friendly
  way. Eg. `<command> | curl -F 'sprunge=<-' http://sprunge.us` for sprunge.
  This is the simplest and most lightweight option, but requires remembering
  the specific curl arguments or setting up a shell alias for easy use.
- server/site-specific clients include: [ix](http://ix.io/client),
  [gist](https://github.com/defunkt/gist),
  [fb](https://git.server-speed.net/users/flo/fb/) (paste.xinu.at), ...

See the [archwiki list of pastebin clients](https://wiki.archlinux.org/index.php/List_of_applications/Internet#Pastebin_clients) for other related projects.


## License

Copyright © 2019 Samuel Walladge

Dual licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
