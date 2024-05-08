use elysium_core::env::release;

pub async fn init() {
	// Initialize tracing and logging
	// Print local CLI version
	println!("{}", release());
}
