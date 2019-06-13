#!/usr/bin/bash

# quick script to sanity check the default config and example config

set -ex

BIN="cargo run --"

$BIN -c NONE dump-config
$BIN -c default_config.toml dump-config
$BIN -c example_config.toml dump-config
$BIN -c example_config.toml list
$BIN -c example_config.toml list-backends
