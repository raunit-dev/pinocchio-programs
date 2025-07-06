#[cfg(test)]
mod tests {

    use mollusk_svm::{
        Mollusk,
        result::{Check, ProgramResult},
    };
    use sol_trasnfer::{ID, instructions::shared::TransferSolInstructionData};

    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    //        * Integration Tests: This file contains integration tests for the Solana program, using the mollusk-svm crate to simulate the Solana runtime environment.
    //    * Mollusk::new(): Initializes a simulated Solana Virtual Machine (SVM) instance, loading your program.
    //    * keyed_account_for_system_program(): Helper to get the System Program's public key and account data.
    //    * Test Setup: Both test_transfer_sol_with_program and test_transfer_sol_with_cpi follow a similar pattern:
    //        * Define payer and recipient public keys and their initial account states (AccountSharedData).
    //        * Create TransferSolInstructionData with the amount to transfer.
    //        * Construct an Instruction object, including:
    //            * The program ID (PROGRAM_ID).
    //            * The instruction data (prefixed with 0 for direct transfer, 1 for CPI transfer).
    //            * The AccountMeta list, specifying which accounts are involved and their properties (signer, writable, etc.).
    //    * mollusk.process_and_validate_instruction(): This is the core of the test. It executes the instruction on the simulated SVM and allows you to define Checks to assert the expected state of accounts
    //      after the instruction execution (e.g., final lamport balances).
    //    * assert!(result.program_result == ProgramResult::Success): Asserts that the instruction executed successfully.

    #[test]
    fn test_transfer_sol_with_program() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/sol_trasnfer");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer: Pubkey = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &PROGRAM_ID);

        let receipent = Pubkey::new_from_array([0x03; 32]);
        let receipent_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = TransferSolInstructionData {
            amount: 1000000000u64,
        };

        // create counter instruction
        {
            let mut data = vec![0];
            data.extend_from_slice(unsafe {
                core::slice::from_raw_parts(
                    &ix_data as *const TransferSolInstructionData as *const u8,
                    core::mem::size_of::<TransferSolInstructionData>(),
                )
            });

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(payer, true),
                    AccountMeta::new(receipent, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (payer, payer_account.clone().into()),
                        (receipent, receipent_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&payer).lamports(0).build(),
                        Check::account(&receipent).lamports(1000000000).build(),
                    ],
                );
            assert!(result.program_result == ProgramResult::Success);
        }
    }

    #[test]
    fn test_transfer_sol_with_cpi() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/sol_trasnfer");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let payer = Pubkey::new_from_array([0x02; 32]);
        let payer_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let receipent = Pubkey::new_from_array([0x03; 32]);
        let receipent_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = TransferSolInstructionData {
            amount: 1000000000u64,
        };

        // create counter instruction
        {
            let mut data = vec![1];
            data.extend_from_slice(unsafe {
                core::slice::from_raw_parts(
                    &ix_data as *const TransferSolInstructionData as *const u8,
                    core::mem::size_of::<TransferSolInstructionData>(),
                )
            });

            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &data,
                vec![
                    AccountMeta::new(payer, true),
                    AccountMeta::new(receipent, false),
                    AccountMeta::new_readonly(system_program, false),
                ],
            );

            let result: mollusk_svm::result::InstructionResult = mollusk
                .process_and_validate_instruction(
                    &instruction,
                    &[
                        (payer, payer_account.clone().into()),
                        (receipent, receipent_account.clone().into()),
                        (system_program, system_account.clone()),
                    ],
                    &[
                        Check::success(),
                        Check::account(&payer).lamports(0).build(),
                        Check::account(&receipent).lamports(1000000000).build(),
                    ],
                );
            assert!(result.program_result == ProgramResult::Success);
        }
    }
}
