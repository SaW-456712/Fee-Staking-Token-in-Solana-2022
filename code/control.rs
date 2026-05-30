use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair_file, Signer, transaction::Transaction};
use spl_token_2022::{ID as T2022, extension::{transfer_fee::instruction::set_transfer_fee, interest_bearing_mint::instruction::update_rate_interest_bearing_mint}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Use: cargo run --bin control [fee|rate] [value]");
        return Ok(());
    }

    let action = &args[1];
    let val: i16 = args[2].parse().expect("Invalid number");

    let client = RpcClient::new_with_commitment("https://api.devnet.solana.com".to_string(), CommitmentConfig::confirmed());
    let admin = read_keypair_file("admin_keypair.json").expect("No admin file");
    let mint = read_keypair_file("token_keypair.json").expect("No token file");

    let mut ixs = vec![];

    if action == "fee" {
        let ix = set_transfer_fee(&T2022, &mint.pubkey(), &admin.pubkey(), &[], val as u16, 5000_000_000)?;
        ixs.push(ix);
        println!("Changing fee to {}%...", val as f64 / 100.0);
    } else if action == "rate" {
        let ix = update_rate_interest_bearing_mint(&T2022, &mint.pubkey(), &admin.pubkey(), &[], val)?;
        ixs.push(ix);
        println!("Changing staking rate to {}%...", val as f64 / 100.0);
    } else {
        println!("Unknown action. Use 'fee' or 'rate'");
        return Ok(());
    }

    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&ixs, Some(&admin.pubkey()), &[&admin], blockhash);
    let sign = client.send_and_confirm_transaction(&tx)?; println!("Success! Tx: {}", sign);

    Ok(())
}
