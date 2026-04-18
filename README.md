# eui64tool                                                  
                                                               
A CLI multitool for working with EUI-64 — convert MAC
addresses to IPv6 EUI-64 suffixes, reverse them back, inspect
live interfaces, or decode a full IPv6 address.

## What is EUI-64?

IPv6 uses EUI-64 to derive interface identifiers from MAC
addresses. The process inserts `ff:fe` in the middle of the
MAC and flips the universal/local bit, producing a 64-bit
suffix used in SLAAC addresses. `eui64tool` makes this
conversion (and its reverse) easy to work with from the
command line.

## Install

```sh
cargo install --path .
```
Nix users: a flake.nix is included. direnv allow sets up the
dev environment automatically.

Usage

No arguments — list all network interfaces with their MAC
addresses and EUI-64 suffixes:
$ eui64tool
IPv6 EUI-64 suffix for eth0: :a8bb:ccff:fedd:eeff
IPv6 EUI-64 suffix for wlan0: :1034:56ff:fe78:9abc

MAC address — convert to EUI-64 suffix:
$ eui64tool aa:bb:cc:dd:ee:ff
IPv6 EUI-64 suffix for aa:bb:cc:dd:ee:ff: :a8bb:ccff:fedd:eeff

EUI-64 suffix — reverse back to MAC:
$ eui64tool a8bb:ccff:fedd:eeff
MAC address for :a8bb:ccff:fedd:eeff: aa:bb:cc:dd:ee:ff

Full IPv6 address — extract and reverse the interface
identifier:
$ eui64tool fe80::a8bb:ccff:fedd:eeff
MAC address for fe80::a8bb:ccff:fedd:eeff: aa:bb:cc:dd:ee:ff

Interface name — look up a live interface by name:
$ eui64tool eth0
IPv6 EUI-64 suffix for eth0: :a8bb:ccff:fedd:eeff

Built with

- https://crates.io/crates/getifs — interface enumeration
- https://crates.io/crates/regex — input classification
