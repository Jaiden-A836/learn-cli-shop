# Rust CLI Practice Repository

This repository serves as a dedicated playground for learning Rust programming through hands-on development. The project focuses on building a modular command-line application while exploring foundational language concepts, including pattern matching, error handling, module organization, and CLI input processing.

---

## Project Overview

The primary objective of this repository is to gain practical experience writing idiomatic Rust code. Rather than building a production-ready application, the focus remains on learning language mechanics, project structuring, and cargo tooling.

### Key Learning Focus Areas

- **Control Flow & Types:** Practicing `enum` variants, `match` control flow, and vector processing.
- **Error Handling:** Implementing custom error types and managing terminal input errors using standard `Result` types.
- **Module Architecture:** Moving code out of `src/main.rs` and organizing business logic into separate, focused modules.
- **External Dependencies:** Integrating crates such as `strum` and `strum_macros` for enum iteration and string conversions.

> [!NOTE]
> This repository is strictly for practice and experimental learning. Code structures may evolve as new Rust patterns and best practices are explored.

---

## Repository Structure

The project follows a standard Cargo module layout to separate command definitions, error handling, and the interactive event loop:

```text
.
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── commands.rs     # Command definitions and strum enum attributes
│   ├── errors.rs       # Custom CommandError types and handlers
│   ├── main_loop.rs    # Interactive REPL prompt and command parsing logic
│   └── main.rs         # Application entry point and module declarations
└── README.md
```

---

## Getting Started

Follow these steps to build and run the application locally on your machine.

### Prerequisites

Ensure you have the Rust toolchain installed. If `cargo` is not available, install it via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

### Installation & Execution

1. Clone the repository to your local directory:
   ```bash
   git clone [https://github.com/your-username/cli-shop.git](https://github.com/your-username/cli-shop.git)
   cd cli-shop
   ```

2. Compile and run the application using Cargo:
   ```bash
   cargo run
   ```

3. Execute tests to verify module behavior:
   ```bash
   cargo test
   ```

> [!TIP]
> Use `cargo check` during active development to quickly validate syntax and type safety without generating release binaries.
