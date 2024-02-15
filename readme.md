# build 
    cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist

# config 
    //solana config get
    solana config set --url https://api.devnet.solana.com

# gen key paire
    solana-keygen new --outfile /path/to/your/program-keypair.json

# check balance
    solana balance

# deploy 
    solana program deploy --program-id /path/to/your/program-keypair.json dist/your_program.so

# check program
    solana address -k /path/to/your/program-keypair.json


# check account
    solana account program-id