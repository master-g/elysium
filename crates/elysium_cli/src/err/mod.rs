#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("bad argument: {0}")]
	BadArgument(String),
}
