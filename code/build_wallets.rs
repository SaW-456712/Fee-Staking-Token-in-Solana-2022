use solana_sdk::signature::{Keypair, write_keypair_file, Signer};
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let admin = Keypair::new();
	let token = Keypair::new();
	write_keypair_file(&admin, "admin_keypair.json").map_err(|e| format!("Admin save error: {}", e))?;
	write_keypair_file(&token, "token_keypair.json").map_err(|e| format!("Token save error: {}", e))?;
	let p_key = bs58::encode(admin.to_bytes()).into_string();

	println!("Admin PK: {}", admin.pubkey());
	println!("Token PK: {}", token.pubkey());
	println!("Phantom Key: {}", p_key);
	Ok(())
}
