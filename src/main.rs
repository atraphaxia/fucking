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



fn main()
{
	let cli = Cli::parse();
}
