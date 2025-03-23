use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
	/// Runs a review on a port
	Review {
		/// A repository URL to review
		url: String,

		/// Whether to skip cloning
		#[clap(long, short, action)]
		skip_clone: bool
	},
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,
}
