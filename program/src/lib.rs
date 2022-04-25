use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed},
    pubkey::Pubkey, system_program,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let pda = next_account_info(account_info_iter)?;
    let recipeint = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    msg!("The pad account info {:?}", pda);
    assert!(payer.is_writable);
    assert!(payer.is_signer);
    assert!(pda.is_writable);
    assert_eq!(pda.owner, &system_program::ID);
    assert!(system_program::check_id(system_program.key));
    let pda_phrase = &instruction_data[0..instruction_data.len() - 1];
    let bump_seed = instruction_data[instruction_data.len() - 1];

    msg!(
        "The pda phrase {:?}",
        String::from_utf8(pda_phrase.to_vec())
    );
    msg!("The bump seed  {:?}", bump_seed);

    let pda_seeds: &[&[u8]] = &[pda_phrase, &[bump_seed]];
    let expected_pda = Pubkey::create_program_address(pda_seeds, program_id)?;
    assert_eq!(pda.key, &expected_pda);

    let lamports = 8000000000;
    let instruction =
        solana_program::system_instruction::transfer(&expected_pda, recipeint.key, lamports);
    let new_accs = [pda.clone(), recipeint.clone()];
    invoke_signed(&instruction, &new_accs, &[pda_seeds])?;

    Ok(())
}
