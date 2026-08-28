monosecret_derive::declare_secrets!("monosecret.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let resolved = Monosecret::builder()
		.with_provider("keyring://")
		.with_profile("development")
		.with_reason("start application")
		.prompt_missing(true)
		.load()?;

	println!("Database: {}", resolved.secrets.database_url);

	Ok(())
}
