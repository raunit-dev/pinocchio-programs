# Solana Account Data Example

This project is a simple Solana program that demonstrates how to create and store data in an account. It is built using the Pinocchio framework, which simplifies the process of developing Solana programs in Rust.

## Codebase Overview

The codebase is structured as follows:

*   `src/lib.rs`: The main entry point of the program.
*   `src/processor.rs`: The instruction router that handles all incoming instructions.
*   `src/instructions/create.rs`: The instruction for creating a new account and storing address information.
*   `src/state/address_info.rs`: The data structure for storing address information.
*   `tests/account_data.rs`: The test suite for the program.

### `src/lib.rs`

This file defines the program's entry point, `process_instruction`, which is responsible for handling all instructions sent to the program. It also declares the program's on-chain address (public key).

### `src/processor.rs`

This file contains the `process_instruction` function, which acts as an instruction router. It deserializes the instruction data and calls the appropriate instruction handler based on the instruction discriminator.

### `src/instructions/create.rs`

This file defines the `Create` instruction, which is responsible for creating a new account and storing address information in it. It performs the following steps:

1.  Validates the accounts passed into the instruction.
2.  Deserializes the instruction data into a `CreateAddressInfoInstructionData` struct.
3.  Creates a new account using the `pinocchio_system::instructions::CreateAccount` instruction.
4.  Serializes the address information into the newly created account's data.

### `src/state/address_info.rs`

This file defines the `AddressInfo` struct, which represents the data structure for storing address information. The `#[repr(C)]` attribute ensures that the struct's layout is consistent across different architectures, which is important for on-chain data.

### `tests/account_data.rs`

This file contains the test suite for the program. It uses the `mollusk-svm` testing framework to test the `Create` instruction.

## Building and Testing

To build and test the program, you will need to have the following installed:

*   Rust
*   Solana CLI

Once you have the prerequisites installed, you can build the program using the following command:

```
cargo build-bpf
```

To run the tests, use the following command:

```
cargo test-bpf
```

## Usage

To use the program, you will need to deploy it to a Solana cluster. Once the program is deployed, you can interact with it using a client-side script. The client-side script will need to create a transaction with an instruction to call the `Create` instruction in the on-chain program.
