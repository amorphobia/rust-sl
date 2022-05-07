# Rust SL

A rust port of [sl](https://github.com/mtoyoda/sl).
SL (Steam Locomotive) runs across your terminal when you type "sl" as you meant to type "ls". It's just a joke command, and not useful at all.

## Installing

### Using [Scoop](https://scoop.sh/)

```PowerShell
scoop bucket add siku https://github.com/amorphobia/siku
scoop install rust-sl
```

### Prebuilt Binaries

Find and download from [release page](https://github.com/amorphobia/rust-sl/releases).

## Usage

```PowerShell
sl [OPTIONS]
```

## Options

```text
-a            An accident is occurring. People cry for help
-c            C51 appears instead of D51
-F            It flies like the galaxy express 999
-h, --help    Print help information
-l            Little version
```

## License

The original code was written in c by Toyoda Masashi. See [Original License](#original-license-c-version) below.
The modified (rust) code is opensourced under [AGPL-3.0](https://github.com/amorphobia/rust-sl/blob/master/LICENSE).

## Original License (c version)

Copyright 1993,1998,2014 Toyoda Masashi (mtoyoda@acm.org)

Everyone is permitted to do anything on this program including copying,
modifying, and improving, unless you try to pretend that you wrote it.
i.e., the above copyright notice has to appear in all copies.
THE AUTHOR DISCLAIMS ANY RESPONSIBILITY WITH REGARD TO THIS SOFTWARE.
