
# Solana SOL Transfer Example

This project demonstrates two distinct methods for transferring SOL (the native token of the Solana blockchain) between accounts within a Solana program. It serves as an educational tool to highlight the correct, secure way to perform transfers versus a direct, unsafe method.

## Key Concepts

The core of this project is to illustrate the difference between:

1.  **Direct Balance Manipulation:** Modifying the lamport balances of accounts directly within the program logic. This is generally **highly discouraged and unsafe** as it bypasses the Solana runtime's built-in checks and can lead to critical vulnerabilities, such as creating or destroying SOL if not handled with absolute precision.

2.  **Cross-Program Invocation (CPI):** Creating a `Transfer` instruction and invoking the official Solana **System Program** to execute the transfer. This is the **secure, standard, and recommended** method. It delegates the sensitive task of altering account balances to a trusted, audited, and core component of the Solana blockchain.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── instructions
│   │   ├── mod.rs
│   │   ├── shared.rs
│   │   ├── transfer_sol_with_cpi.rs
│   │   └── transfer_sol_with_program.rs
│   ├── lib.rs
│   └── processor.rs
└── tests
    └── sol-transfer.rs
```

-   `src/lib.rs`: The main entry point of the program.
-   `src/processor.rs`: Contains the core instruction processing logic, routing instructions to the appropriate handlers.
-   `src/instructions/`: This module contains the logic for the different transfer methods.
    -   `transfer_sol_with_program.rs`: Implements the **unsafe** direct balance manipulation.
    -   `transfer_sol_with_cpi.rs`: Implements the **safe** transfer using a CPI call to the System Program.
    -   `shared.rs`: Contains shared data structures used by both transfer instructions.
-   `tests/sol-transfer.rs`: Contains integration tests that verify the functionality of both transfer methods.

## Building and Testing

To build the project and run the tests, you can use the following standard Cargo commands:

```bash
# Build the project
cargo build

# Run the integration tests
cargo test
```

## Usage

The program has a single entry point but uses the first byte of the instruction data to differentiate between the two transfer methods:

-   **Instruction Byte `0`**: Routes to `transfer_sol_with_program` (the direct, unsafe method).
-   **Instruction Byte `1`**: Routes to `transfer_sol_with_cpi` (the secure, CPI method).

The instruction data is expected to be in the following format:

| Byte(s) | Description                |
|---------|----------------------------|
| 1       | Instruction discriminator  |
| 8       | Amount of lamports (u64)   |

This project is intended for educational purposes to demonstrate best practices in Solana development. When building your own programs, **always use CPI to the System Program for SOL transfers**.
