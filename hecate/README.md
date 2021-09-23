# hecate
A dynamically recompiled ARM Cortex-A9 CPU emulator, which sole purpose is to be a learning project.
This CPU implements the ARMv7-A architecture.

## About
hecate aims to provide an easy-to-use interface to emulate the ARM-Cortex A9 CPU (in future there may be support for other ARM CPU's and architectures as well).
Furthermore it will be possible to have multiple target architectures, this is made possible by the fact that the dynamic recompiler is not compiling directly to the target architecutre but first into an IL designed by ourselves, and then compiled further to the target machine code.
Target architectures we aim to support are x86-64 and AArch64.

## Building, Testing and Running
TODO

## License
hecate is distributed like Anubis under the terms of either the Apache License (Version 2.0) or the MIT license, at the user's choice.
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Contributions to the hecate must be made under the terms of both licenses.