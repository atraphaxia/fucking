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
}



fn main()
{
	let cli = Cli::parse();
}
