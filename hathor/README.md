# hathor
hathor is the whole part of gpu emulation used by the anubis Vita emulator.

## About
hathor is the whole part of the GPU emulation used by the anubis emulator.
The goal of hathor is to provide a simple interface to emulate the GPU of the Vita (PowerVR SGX543 MP4+).
With the help of tools like wgpu, the rendering is done on the host PC.
The shaders are decompiled from their binary form into the SPIR-V intermediate shading language and then further compiled for the host GPU.
It is just like hecate a learning project to learn more about what it actually looks like behind this whole facade.

## Building, Testing and Running
TODO

## License
hathor is distributed like Anubis under the terms of either the Apache License (Version 2.0) or the MIT license, at the user's choice.
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Contributions to hathor must be made under the terms of both licenses.