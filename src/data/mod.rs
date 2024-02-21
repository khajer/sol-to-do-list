// use borsh::{BorshDeserialize, BorshSerialize};
// pub mod data {
//     #[derive(BorshSerialize, BorshDeserialize, Debug)]
//     pub struct TodoItem {
//         id: u32,
//         description: String,
//         completed: bool,
//     }
// }
pub struct TodoItem {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}
