# Data Compat MC
[![Join on discord!](https://shields.io/discord/925832475912065024)](https://discord.com/invite/HC82fwYXW5)
Data tool for the [Falcon MC](https://github.com/GrizzlT/FalconMC) project.

## Description
DataCompatMC is a `cli` tool designed to help deal with the many different versions of Minecraft. Its main purpose is measuring and fixing compatibility between protocol-defined values for the different assets in the game.

## Features
We aim to support two main operations:
- Provide compatibility support between different schematic versions
- Autogenerate conversion code for the different protocol versions of `FalconMC`

Currently this tool only parses generated block data by the [Minecraft data generators](https://wiki.vg/Data_Generators) and compacts it down to a lossless, minimal format.

## Usage
Clone the project and build it using `cargo build --release`.
Use `data-compat-mc --help` for further information.

##  Contributing

Please feel free to help out in any way possible.

- [Issues](https://github.com/GrizzlT/DataCompatMC/issues)

- [Pull requests](https://github.com/GrizzlT/DataCompatMC/pulls)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.


##  License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)

* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.