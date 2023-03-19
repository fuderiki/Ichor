# Ichor
Ichor: High-performance AI game algorithms in Rust, abstracting complexity for effortless integration. Elegant C# and C++ bindings for your favorit game engine, empowering game devs with divine AI prowess.


Ichor is a high-performance AI game algorithms library written in Rust, inspired by the blood of the gods in Greek mythology. The goal of Ichor is to abstract away the complexity of AI implementations in a concurrency-free and efficient Rust codebase, allowing game developers to easily integrate powerful AI solutions into their projects.

## Features

- Efficient, concurrency-free AI algorithms in Rust
- Seamless integration with game engines through dedicated bindings (C# for Unity and C++ for Unreal)
- CPU, concurrent, multithreading, and GPU computation support
- Modular and easy-to-extend design

## Structure

- `src/`
  - `cpu/`: CPU implementation of AI algorithms
  - `multithreading/`: Multithreading implementation of AI algorithms
  - `gpu/`: GPU implementation of AI algorithms using WGPU
  - `lib.rs`: Exports the modules for usage in bindings

## Getting Started

To use Ichor as a dependency in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
ichor = { git = "https://github.com/fuderiki/ichor.git" }
```
Then, in your Rust code, import the Ichor library:
```rust
extern crate ichor;
```
For usage examples, see the documentation.

## Bindings

The following repositories contain the bindings for popular game engines:

- [Ichor C# Bindings](https://github.com/fuderiki/ichor-CSHARP-bindings)
- [Ichor C++ Bindings](https://github.com/fuderiki/ichor-CPP-bindings)

Please refer to the respective repositories for instructions on using Ichor with Unity or Unreal Engine.

## Documentation

For detailed documentation and examples, visit [Ichor Documentation](https://fuderiki.github.io/ichor-docs/). 

## License

This project is licensed under the [MIT License](LICENSE).
