mod foo;
mod version;

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use elysium_core::{cst::LOGO, env};

use crate::{
	err::Error,
	logging::{CustomEnvFilter, CustomEnvFilterParser},
};

const INFO: &str = r#"
MG's Rust Elysium
"#;

#[derive(Parser, Debug)]
#[command(name = "Elysium command-line interface", bin_name = "elysium")]
#[command(author, version, about = INFO, before_help = LOGO)]
#[command(disable_version_flag = true, arg_required_else_help = true)]
struct Cli {
	#[arg(help = "Configuration file to use")]
	#[arg(env = "ELYSIUM_CONFIG", short, long)]
	#[arg(default_value = "elysium.config.toml")]
	#[arg(global = true)]
	config: String,

	#[arg(help = "The logging level")]
	#[arg(env = "ELYSIUM_LOG", short = 'l', long = "log")]
	#[arg(default_value = "info")]
	#[arg(value_parser = CustomEnvFilterParser::new())]
	#[arg(global = true)]
	log: CustomEnvFilter,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
enum Commands {
	#[command(about = "Test command")]
	Foo(foo::FooCommandArguments),
	#[command(about = "Okey mahjong command")]
	Okey,
	#[command(about = "Print version information")]
	Version,
}

pub(crate) async fn prepare(
	_config: &str,
	log: CustomEnvFilter,
) -> Result<Option<tracing_appender::non_blocking::WorkerGuard>, Error> {
	let guard =
		crate::logging::builder().with_filter(log).with_file_appender(".logs".into()).build();

	Ok(guard)
}

pub async fn init() -> ExitCode {
	env::init().await;

	let args = Cli::parse();

	// version command is special
	if let Some(Commands::Version) = args.command {
		version::init().await;
		return ExitCode::SUCCESS;
	}

	let _guard = prepare(&args.config, args.log).await.unwrap();

	let output = match args.command {
		Some(Commands::Foo(args)) => foo::init(args).await,
		_ => Ok(()),
	};

	if let Err(e) = output {
		error!("{}", e);
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}
