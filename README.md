# proquint-rs [![Build Status](https://travis-ci.org/christian-blades-cb/proquint-rs.svg?branch=master)](https://travis-ci.org/christian-blades-cb/proquint-rs) [![Latest Version](https://img.shields.io/crates/v/proquint.svg)](https://crates.io/crates/proquint) [![Docs](https://docs.rs/proquint/badge.svg)](https://docs.rs/proquint)

Proquints are readable, pronouncable representations of identifiers. This is a Rust library for converting between them.

## Proquints

Read more about proquints: https://arxiv.org/html/0901.4016

Ex. the ip address `127.0.0.1` is `lusab-babad` in proquint form.

## Usage

``` rust
extern crate proquint;

use proquint::Quintable;
use std::net::Ipv4Addr;

let home = Ipv4Addr::new(127, 0, 0, 1);
assert_eq!(home.to_quint(), "lusab-babad");

assert_eq!(u32::from_quint("rotab-vinat").unwrap(), 3141592653u32);
```
