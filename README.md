# aprshttp
[![Crates.io](https://img.shields.io/crates/v/aprshttp)](https://crates.io/crates/aprshttp) 
[![Docs.rs](https://docs.rs/aprshttp/badge.svg)](https://docs.rs/aprshttp) 
[![Build](https://github.com/Ewpratten/aprshttp/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/aprshttp/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/aprshttp/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/aprshttp/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/aprshttp/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/aprshttp/actions/workflows/audit.yml)


Send one-way APRS position reports using APRS-IS.

```text
aprshttp 0.1.0
Evan Pratten <ewpratten@gmail.com>
One-way APRS position reporting using APRS-IS

USAGE:
    aprshttp [OPTIONS] <callsign> --latitude <latitude> --longitude <longitude>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --latitude <latitude>      Latitude
        --longitude <longitude>    Longitude
    -m, --message <message>        Message to broadcast
    -s, --symbol <symbol>          APRS symbol [default: -]

ARGS:
    <callsign>    Your callsign
```

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install aprshttp
```
