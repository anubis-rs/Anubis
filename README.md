# Anubis
An open-source PS Vita emulator written in Rust.

## About
The PS Vita is a very interesting system, which gave me the reason to write an emulator for this system. Although the main purpose of the project is to serve as a learning project, Anubis tries to emulate the Vita as closely as possible.
The individual components, such as the CPU JIT Compiler or the GPU emulation, are split into individual crates so that they can be easily used in other projects without having to drag the entire Anubis ecosystem into them.

## Components
 - [`anubis`](./anubis): The crate which combines all parts of the emulator
 - [`hathor`](./hathor): GPU emulation crate which organizes all sort of shader recompilation and stuff
 - [`hecate`](./hecate): Implementation of a dynamic recompiler for the CPU and therefore in my eyes the most important crate

## Setup
Coming soon

## Contributing
The project is still at a very early stage of development.
Contributions are welcome everytime.

## Contact
Feel free to reach out to `Lockna#5599` on Discord or you can directly join our [Anubis Discord server](https://discord.gg/Kphvzgwdkz)

## Credits
Coming soon

## License
Anubis is distributed under the terms of either the Apache License (Version 2.0) or the MIT license, at the user's choice.
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Contributions to the Anubis project must be made under the terms of both licenses.
