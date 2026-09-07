# solana-template

A minimalist Solana program template with `cargo generate` support.

## Motivation

This template exists because writing the same boilerplate is tedious.
With **one command**, you can start a new program without writing the skeleton yourself.

## Quick Start

Just execute:
```bash
cargo generate --git https://github.com/red4vy/solana-template --branch feat/instruction
```

## What You Get

```
my-program/
├── Cargo.toml
└── src
    ├── entrypoint.rs
    ├── instruction.rs
    ├── lib.rs
    └── processor.rs
```

## Features in This Branch

This branch extends the base template with enhanced instruction handling.
