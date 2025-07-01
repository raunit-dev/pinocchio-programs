#[cfg(test)]
mod tests {
    use account_data::{
        state::{AddressInfo, CreateAddressInfoInstructionData},
        ID,
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
    fn test_create_account_data() {
        pub fn create_padded_array<const N: usize>(data: &[u8], size: usize) -> [u8; N] {
            let mut result = [0u8; N];
            let copy_size = data.len().min(size).min(N);
            result[..copy_size].copy_from_slice(&data[..copy_size]);
            result
        }
        let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/account_data");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let address_info_pubkey = Pubkey::new_unique();
        let address_info_account = AccountSharedData::new(0, 0, &system_program);

        let ix_data = CreateAddressInfoInstructionData {
            name: create_padded_array(b"Raunit", 50),
            house_number: 69,
            street: create_padded_array(b"kolkata Street", 50),
            city: create_padded_array(b"Main City", 50),
        };

        let ix_data_bytes = bytemuck::bytes_of(&ix_data);

        let data = [vec![0], ix_data_bytes.to_vec()].concat();

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(owner, true),
                AccountMeta::new(address_info_pubkey, true),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (owner, owner_account.into()),
                    (address_info_pubkey, address_info_account.into()),
                    (system_program, system_account),
                ],
                &[
                    Check::success(),
                    Check::account(&address_info_pubkey)
                        .data(ix_data_bytes)
                        .build(),
                ],
            );

        let updated_data = result.get_account(&address_info_pubkey).unwrap();
        let parsed_data = bytemuck::from_bytes::<AddressInfo>(&updated_data.data);

        assert_eq!(parsed_data.name, create_padded_array(b"Raunit", 50));
        assert_eq!(parsed_data.house_number, 69);
        assert_eq!(
            parsed_data.street,
            create_padded_array(b"kolkata Street", 50)
        );
        assert_eq!(parsed_data.city, create_padded_array(b"Main City", 50));

        assert!(result.program_result == ProgramResult::Success);
    }
}
