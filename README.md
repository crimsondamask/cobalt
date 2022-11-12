# cobalt

> **Allen Bradley Ethernet/IP Command Line Utility**

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Contributors](https://img.shields.io/apm/l/cobalt)](https://github.com/crimsondamask/cobalt/graphs/contributors)

## About

Cobalt is a command line utility for communicating with Programmable Logic Controllers. It only supports Allen Bradley CompactLogix for now.

## Use

```
❯ cobalt --help
A command line utility for parsing and reading tags on Allen Bradley CompactLogix PLCs.

Usage: cobalt --address <ADDRESS> <COMMAND>

Commands:
  list        List controller tags
  read-int    Read the INT value of a tag
  read-dint   Read the DINT value of a tag
  read-real   Read the REAL value of a tag
  read-bool   Read the BOOL value of a tag
  write-bool  Write a BOOL value to the specified tag
  write-int   Write an INT value to the specified tag
  write-dint  Write a DINT value to the specified tag
  write-real  Write a REAL value to the specified tag
  help        Print this message or the help of the given subcommand(s)

Options:
  -a, --address <ADDRESS>  PLC address
  -h, --help               Print help information
  -V, --version            Print version information

```

