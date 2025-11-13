use clap::{Parser, Subcommand};



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
		name: String
	}
}



fn main()
{
	let cli = Cli::parse();
}
