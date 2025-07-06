#[cfg(test)]
mod tests {
    use counter::{
        constants::COUNTER_SEED, instructions::CreateCounterInstructionData, state::Counter, ID,
    };
    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };

    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_create_counter_data() {
        // ARRANGE: Part 1 - Setup the environment
        let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/counter"); // Load the compiled program
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program(); // Get the system program
        let owner = Pubkey::new_from_array([0x02; 32]); // Create a fake user/payer public key
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program); // Give the user 1 SOL to pay for things

        // ARRANGE: Part 2 - Setup the counter account
        // Calculate the expected Program Derived Address (PDA) for the counter.
        // This MUST match the logic inside the program.
        let (counter_pubkey, bump) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[COUNTER_SEED], &PROGRAM_ID);
        // The counter account starts with 0 lamports and 0 data. Our program will create it.
        let counter_account = AccountSharedData::new(0, 0, &system_program);

        // ARRANGE: Part 3 - Define the instruction and expected outcome
        let counter_init_state = Counter {
            count: 100u64.to_le_bytes(), // We expect the final count to be 100
        };

        // Build the data for our `Create` instruction
        let ix_data = CreateCounterInstructionData {
            initial_value: counter_init_state.count,
            bump,
        };
        // Serialize the instruction data into raw bytes
        let ix_data_bytes = bytemuck::bytes_of(&ix_data);
        // Prepend the discriminator `0` to the instruction bytes. This is how our processor knows it's a `Create` instruction.
        let data = [vec![0], ix_data_bytes.to_vec()].concat();

        // Build the final Solana instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(owner, true),           // The user/payer is a signer
                AccountMeta::new(counter_pubkey, false), // The counter account is writable
                AccountMeta::new_readonly(system_program, false), // System program is needed for account creation
            ],
        );

        // ACT & ASSERT
        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                // These are the accounts BEFORE the instruction runs
                &[
                    (owner, owner_account.clone().into()),
                    (counter_pubkey, counter_account.clone().into()),
                    (system_program, system_account.clone()),
                ],
                // These are the CHECKS to run AFTER the instruction
                &[
                    Check::success(), // 1. The program must not return an error
                    Check::account(&counter_pubkey).owner(&PROGRAM_ID).build(), // 2. The counter account must now be owned by our program
                    Check::account(&counter_pubkey)
                        .data(bytemuck::bytes_of(&counter_init_state)) // 3. The counter's data must be exactly our expected initial state (100)
                        .build(),
                ],
            );

        // Final manual check (optional but good practice)
        let updated_data = result.get_account(&counter_pubkey).unwrap();
        let parsed_data = bytemuck::from_bytes::<Counter>(&updated_data.data);
        assert_eq!(parsed_data.count, 100u64.to_le_bytes());
    }

    #[test]
    fn test_increase_counter_data() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/counter");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (counter_pubkey, _) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[COUNTER_SEED], &PROGRAM_ID);

        let counter_init_state = Counter {
            count: 100u64.to_le_bytes(),
        };

        {
            let mut counter_account = AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Counter::LEN),
                Counter::LEN,
                &PROGRAM_ID,
            );

            counter_account.set_data_from_slice(bytemuck::bytes_of(&counter_init_state));

            let data = vec![1];

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(owner, true),
                    AccountMeta::new(counter_pubkey, true),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (owner, owner_account.into()),
                        (counter_pubkey, counter_account.into()),
                        (system_program, system_account),
                    ],
                    &[
                        Check::success(),
                        Check::account(&counter_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&counter_pubkey)
                            .data(bytemuck::bytes_of(&Counter {
                                count: 101u64.to_le_bytes(),
                            }))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&counter_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Counter>(&updated_data.data);

            assert_eq!(parsed_data.count, 101u64.to_le_bytes());

            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_decrease_counter_data() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/counter");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (counter_pubkey, _) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[COUNTER_SEED], &PROGRAM_ID);

        let counter_init_state = Counter {
            count: 100u64.to_le_bytes(),
        };

        // increase counter instruction
        {
            let mut counter_account = AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Counter::LEN),
                Counter::LEN,
                &PROGRAM_ID,
            );

            counter_account.set_data_from_slice(bytemuck::bytes_of(&counter_init_state));

            let data = vec![2];

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(owner, true),
                    AccountMeta::new(counter_pubkey, true),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (owner, owner_account.into()),
                        (counter_pubkey, counter_account.into()),
                        (system_program, system_account),
                    ],
                    &[
                        Check::success(),
                        Check::account(&counter_pubkey).owner(&PROGRAM_ID).build(),
                        Check::account(&counter_pubkey)
                            .data(bytemuck::bytes_of(&Counter {
                                count: 99u64.to_le_bytes(),
                            }))
                            .build(),
                    ],
                );

            let updated_data = result.get_account(&counter_pubkey).unwrap();
            let parsed_data = bytemuck::from_bytes::<Counter>(&updated_data.data);

            assert_eq!(parsed_data.count, 99u64.to_le_bytes());

            assert!(result.program_result == ProgramResult::Success);
        }
    }
}
