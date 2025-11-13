use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use jiff::civil::DateTime;



#[derive(Parser)]
#[command(version, about)]
struct Cli
{
	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand)]
enum Commands
{
	Initialize
	{
		path: PathBuf
	},

	Schedule
	{
		#[command(subcommand)]
		command: ScheduleCommands
	}
}

#[derive(Subcommand)]
enum ScheduleCommands
{
	Add
	{
		name: String,
		start: DateTime,
		end: DateTime
	}
}



fn main() -> Result<(), Box<dyn Error>>
{
	let cli = Cli::parse();

	match &cli.command
	{
		Commands::Initialize { path } =>
		{
			gix::init(path)?;
		}

		Commands::Schedule { command } =>
		{
			todo!();
		}
	}

	Ok(())
}
