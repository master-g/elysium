use clap::Args;

use crate::err::Error;

#[derive(Args, Debug)]
pub(super) struct FooCommandArguments {}

pub(super) async fn init(_args: FooCommandArguments) -> Result<(), Error> {
	println!("Foo command executed!");
	Ok(())
}
