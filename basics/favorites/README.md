
# Favorites Program

The Favorites program is a simple Solana program that allows users to store their favorite number, color, and hobbies on the blockchain. It serves as a basic example of how to create, store, and retrieve data in a Program Derived Address (PDA).

## Features

- **Create PDA**: Creates a new PDA for a user to store their favorites.
- **Get PDA**: Retrieves and displays a user's stored favorites.

## Instructions

The program has two main instructions:

### CreatePda

This instruction creates a new PDA for the user and initializes it with their favorite number, color, and a list of hobbies. The PDA is derived from the user's public key and a predefined seed, ensuring that each user has a unique storage account.

**Accounts:**

- `user`: The user's account, which must be a signer.
- `favorites`: The PDA account where the favorites will be stored.

**Instruction Data:**

- `number`: An 8-byte unsigned integer representing the user's favorite number.
- `color`: A 50-byte string for the user's favorite color.
- `hobbies`: An array of 5 fixed-size 50-byte strings for the user's hobbies.
- `bump`: The bump seed used to generate the PDA.

### GetPda

This instruction retrieves the data from a user's PDA and logs their favorite number and color.

**Accounts:**

- `user`: The user's account, which must be a signer.
- `favorites`: The PDA account to retrieve data from.

## Getting Started

### Prerequisites

- Rust
- Solana CLI

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/favorites-program.git
   cd favorites-program
   ```

2. **Build the program:**
   ```bash
   cargo build-bpf
   ```

3. **Deploy the program:**
   ```bash
   solana program deploy target/deploy/favorites.so
   ```

### Usage

To interact with the program, you can use the Solana CLI or create a client-side application. The following is an example of how to call the `CreatePda` instruction using a custom script:

```javascript
// Example client-side code to create a PDA
const instruction = new TransactionInstruction({
  keys: [
    { pubkey: user.publicKey, isSigner: true, isWritable: false },
    { pubkey: favoritesPda, isSigner: false, isWritable: true },
  ],
  programId,
  data: Buffer.from([...]), // Encoded instruction data
});
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
