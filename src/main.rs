use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Example: Fetch the current block height
    match client.get_slot() {
        Ok(slot) => println!("Current slot: {}", slot),
        Err(e) => eprintln!("Error fetching slot: {}", e),
    }
}
