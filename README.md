# Syndicationd

[![CI](https://github.com/ymgyt/syndicationd/actions/workflows/ci.yaml/badge.svg)](https://github.com/ymgyt/syndicationd/actions/workflows/ci.yaml)

![Demo](./demo.gif)

Syndicationd is a TUI feed viewer, based on [feed-rs](https://github.com/feed-rs/feed-rs) and [ratatui](https://github.com/ratatui-org/ratatui).


## Features

* Subscribe feeds(RSS1, RSS2, Atom,...) and browse latest entries 
* Open the entry in a browser


## Install

### Nix

`nix run github:ymgyt/syndicationd`

### Cargo

`cargo install synd-term --locked`


## Usage

### Log file

The log file path is based on [`ProjectDirs::data_dir()`](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir).  
Please refer to the `--log` flag in `synd --help` for the default output destination.  

You can modify the [log directives](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives) using the environment variable `SYND_LOG`. (for example, `SYND_LOG=synd=debug`)

### Theme

The theme can be changed using the `--theme` flag. Please refer to the help for the values that can be specified.

### Backend api

By default, use `https://syndicationd.ymgyt.io` as the [backend api](./crates/synd_api)([hosted on my home Raspberry Pi](https://github.com/ymgyt/mynix/blob/main/homeserver/modules/syndicationd/default.nix)).  
To change the endpoint, specify the `--endpoint` flag

### Clear cache and logs

Authentication credentials are cached. to remove them, execute `synd clear`.

## License

This project is available under the terms of either the [Apache 2.0 license](./LICENSE-APACHE) or the [MIT license](./LICENSE-MIT).
