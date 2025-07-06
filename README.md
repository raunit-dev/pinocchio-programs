# Pinocchio Programs

Welcome to the **Pinocchio Programs** repository! This repository is a collection of Rust and Solana programs, curated to help you learn and experiment with Solana smart contracts, on-chain operations, and foundational concepts in blockchain programming.

---

## Table of Contents

- [Repository Structure](#repository-structure)
- [Programs in `basics/`](#programs-in-basics)
  - [address-onchain](#address-onchain)
  - [close-account](#close-account)
  - [counter-program](#counter-program)
  - [hello-world-solana](#hello-world-solana)
  - [store-your-favs](#store-your-favs)
- [Getting Started](#getting-started)
- [How to Run the Examples](#how-to-run-the-examples)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Repository Structure

```
pinocchio-programs/
└── basics/
    ├── address-onchain/
    ├── close-account/
    ├── counter-program/
    ├── hello-world-solana/
    └── store-your-favs/
```

Each folder in `basics/` contains a self-contained program, generally written in Rust for the Solana blockchain. These are designed as practical, hands-on examples for learning and experimentation.

---

## Programs in `basics/`

### address-onchain

A simple example for exploring how to derive and use addresses on the Solana blockchain. This program likely demonstrates:
- Creating and working with public keys and on-chain addresses.
- Understanding Program Derived Addresses (PDAs).
- Basic Solana account management.

### close-account

Shows how to safely close Solana accounts and reclaim lamports (Solana’s native tokens) to a specified recipient wallet. This program typically covers:
- The process of closing accounts programmatically.
- Safe transfer of remaining balances.
- Best practices to avoid account rent and unnecessary storage.

### counter-program

A classic Solana example implementing a persistent on-chain counter. This program demonstrates:
- State management in Solana smart contracts.
- Reading and updating account data.
- Transaction processing and instruction handling.

### hello-world-solana

The canonical "Hello, World!" for Solana smart contracts. This is a great starting point for:
- Understanding the minimum structure of a Solana program.
- Deploying and invoking basic instructions.
- Logging and debugging output from on-chain programs.

### store-your-favs

A sample application for storing and retrieving user preferences or favorite items on-chain. This example helps you learn:
- How to serialize and deserialize data in Solana accounts.
- Managing user input and storing custom data structures.
- Accessing and updating multiple user accounts.

---

## Getting Started

### Prerequisites

- **Rust** (with the `solana` toolchain): [Install Rust](https://rustup.rs/)
- **Solana CLI tools**: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- **Git**: To clone the repository

### Clone the Repository

```bash
git clone https://github.com/raunit-dev/pinocchio-programs.git
cd pinocchio-programs/basics
```

---

## How to Run the Examples

Each program in `basics/` is generally a standalone Solana program. To build and deploy one, navigate to its directory and use Cargo:

```bash
cd basics/counter-program
cargo build-bpf  # or 'cargo build-sbf' for Solana's Sealevel backend

# Deploy to localnet:
solana-test-validator    # In a new terminal, start the validator
solana program deploy ./target/deploy/counter_program.so
```

> **Note:** Follow the specific README or comments inside each program directory for more detailed instructions, as some examples may require additional setup or client scripts.

---

## Contributing

Contributions are welcome! If you'd like to add new examples, fix bugs, or suggest improvements:

1. **Fork** the repository.
2. **Create a branch** for your changes.
3. **Commit** your updates with clear messages.
4. **Push** to your fork and open a Pull Request.

Please ensure your example includes clear comments and, if possible, a brief README or usage note inside the program’s directory.

---

## License

This repository currently does **not** specify a license. Please contact the owner for questions about usage, redistribution, or commercial use.

---

## Contact

Maintained by [raunit-dev](https://github.com/raunit-dev).

For questions, suggestions, or collaboration, open an issue or reach out via GitHub.

---

Happy building on Solana! 🚀
