# A Metropolis-Hastings Search on the Local Complementations of a Graph

A Metropolis-Hastings (MH) search through the space of local complementations (LC) of a
graph(state). The goal is to integrate this into Cabaliser.

## Installation

- The header file is located at `./c_lib/include/lcmh.h`.
- Building the dynamic and static libraries can be done via `make build`; the files will
be located in `./c_lib/libs/`. This requires a Rust toolchain to be installed.

### Build Requirements and Dependencies

- Currently, it requires the Rust 2024 edition with a Rust version 1.85.0 or higher; this
  was just the default when I started this here; we can most likely lower this if
  required.
- There are two direct Rust crate dependencies at the moment: `rand` and `rand_pcg`; this
  results at the moment in the following dependency tree:
  ```text
  lcmh v0.1.0
  ├── rand v0.10.3
  │   ├── getrandom v0.4.3
  │   │   ├── cfg-if v1.0.5
  │   │   ├── libc v0.2.189
  │   │   └── rand_core v0.10.1
  │   └── rand_core v0.10.1
  └── rand_pcg v0.10.2
      └── rand_core v0.10.1
  ```
  If necessary, there are ways to get rid of the dependencies:
  - If we already use already some other random number generator that exposes a C
    interface (e.g., via the GNU Scientific Library), we can also just use that.
  - We can implement our own (not so good) random number generator.

## Documentation and Examples

- The documentation in the header file is currently missing, however, most of it already
  exists (and has just not been copied over yet): Run `cargo doc` in `./lcmh/` to generate
  `lcmh/target/doc/lcmh/c_interface/index.html` and view it in a browser. The functions
  and structs there are the same as the ones in the header files, but with Rust syntax
  instead of C syntax.
- There is an example in `./examples/` that shows its intended usage.

## Testing and Bugs

I have not done thorough testing yet, so there are probably bugs.

## Near-Future Work

- The number of single qubits Cliffords can be improved (cf. note in
  `./lcmh/src/clifford_stack.rs`).
- Document what things have to be "freed".
- Do some testing.
- Anything else that we'll need to integrate this into Cabaliser or have more
  sophisticated cost functions (e.g., expose more methods on the graph that can be used in
  the cost function).

## License

This project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).
