use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_program,
};
use common::InstructionData;
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

    assert!(payer.is_writable);
    assert!(payer.is_signer);
    assert!(pda.is_writable);
    assert_eq!(pda.owner, &system_program::ID);
    assert!(system_program::check_id(system_program.key));

    let instruction_data = InstructionData::from_bytes(instruction_data);
    msg!("Instrcution data {:?}", instruction_data);

    let seed = instruction_data.seed;
    let bump_seed = instruction_data.bump_seed;
    let lamports = instruction_data.lamports;

    let pda_seeds: &[&[u8]] = &[seed.as_bytes(), &[bump_seed]];
    let expected_pda = Pubkey::create_program_address(pda_seeds, program_id)?;
    assert_eq!(pda.key, &expected_pda);

    let instruction =
        solana_program::system_instruction::transfer(&expected_pda, recipeint.key, lamports);
    let new_accs = [pda.clone(), recipeint.clone()];
    invoke_signed(&instruction, &new_accs, &[pda_seeds])?;

    Ok(())
}
