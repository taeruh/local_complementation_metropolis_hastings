# A Metropolis-Hastings Search on the Local Complementations of a Graph

A Metropolis-Hastings (MH) search through the space of local complementations (LC) of a
graph(state). The goal is to integrate this into Cabaliser.

## Installation

- The header file is located at `c_lib/include/lcmh.h`.
- Building the dynamic and static libraries can be done via `make c_libraries`; the files
  will be located in `c_lib/libs/`. This requires a Rust toolchain to be installed.
- The python wrapper package can be installed (assuming you are in a virtual environment
  with pip installed) via `make`.

### Build Requirements and Dependencies

- Currently, it requires the Rust 2024 edition with a Rust version 1.85.0 or higher; this
  was just the default when I started this here; we can most likely lower this if
  required.
- There are three direct Rust crate dependencies at the moment: `rand`, `rand_pcg` and
  `rustc-hash`; this results at the moment in the following dependency tree:
  ```text
  lcmh v0.1.0
  ├── rand v0.10.3
  │   ├── getrandom v0.4.3
  │   │   ├── cfg-if v1.0.5
  │   │   ├── libc v0.2.189
  │   │   └── rand_core v0.10.1
  │   └── rand_core v0.10.1
  ├── rand_pcg v0.10.2
  │   └── rand_core v0.10.1
  └── rustc-hash v2.1.3
  ```
  If necessary, there are ways to get rid of the dependencies:
  - If we already use already some other random number generator that exposes a C
    interface (e.g., via the GNU Scientific Library), we can also just use that.
  - We can implement our own (not so good) random number generator.
  - `rustc-hash` is only used to replace the built-in hashing function by a faster (but
    not cryptographically secure) one.

## Documentation and Examples

- The documentation in the header file (and the python wrapper) is currently missing,
  however, most of it already exists (and has just not been copied over yet). See the
  comment at the top in `c_lib/include/lcmh.h` for more information on how to get the
  documentation.
- There is an example in `example/` in `python_wrapper/examples/` that shows the intended
  usage.

## License

This project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).
