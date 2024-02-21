use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction); // main function
mod data;

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello Solana! (From Rust!)");

    let todo_items = data::TodoItem {
        id: 1,
        description: "test".to_string(),
        completed: false,
    };

    Ok(())
}

// mod data {
//     pub struct TodoItem {
//         pub id: u32,
//         pub description: String,
//         pub completed: bool,
//     }
// }
