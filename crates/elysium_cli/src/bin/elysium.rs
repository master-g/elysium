use std::{future::Future, process::ExitCode};

use elysium_core::cst;

fn main() -> ExitCode {
	// Initialize the commnd line
	with_enough_stack(elysium_cli::cli::init())
}

/// Rust's default thread stack size of 2MiB doesn't allow sufficient recursion depth.
fn with_enough_stack<T>(fut: impl Future<Output = T> + Send) -> T {
	// Start a Tokio runtime with custom configuration
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.max_blocking_threads(*cst::RUNTIME_MAX_BLOCKING_THREADS)
		.thread_stack_size(*cst::RUNTIME_STACK_SIZE)
		.thread_name("elysium-worker")
		.build()
		.unwrap()
		.block_on(fut)
}
