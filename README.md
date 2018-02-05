[![Build Status](https://travis-ci.org/christian-blades-cb/proquint-rs.svg?branch=master)](https://travis-ci.org/christian-blades-cb/proquint-rs)

# proquint-rs

A rust library for converting to/from proquints.

## Proquints

Proposal: https://arxiv.org/html/0901.4016

A proquint is a pronouncable representation of an identifier.

Ex. the ip address 127.0.0.1 is lusab-babad in proquint form.

## Usage

``` rust
extern crate proquint;

use proquint::Quintable;
use std::net::Ipv4Addr;

let home = Ipv4Addr::new(127, 0, 0, 1);
assert_eq!(home.to_quint(), "lusab-babad");

assert_eq!(u32::from_quint("rotab-vinat").unwrap(), 3141592653u32);
```
