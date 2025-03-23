use serde::{Serialize, Deserialize};
use clap::Parser;
use git2::{Repository as GitRepository};
use log::{info, warn, error};

mod args;
use args::{Args, Command};

mod contract;
use contract::{NoGitKeepContract, ReadmeContract, AssetsContract, LicenseContract, ContractResult, ReviewContract};

#[derive(Serialize, Deserialize, Debug)]
struct Collaborator {
	url: String,
	username: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
	name: String,
	url: String,
	
	#[serde(rename = "current-maintainers")]
	current_maintainers: Vec<Collaborator>,
	
	#[serde(rename = "past-maintainers")]
	#[serde(default)]
	past_maintainers: Vec<Collaborator>
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
	key: String,
	name: String,
	description: String,
	emoji: String	
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Platform {
	Linux,
	Windows,
	Ios,
	Android,
	Macos,
	Agnostic
}

#[derive(Serialize, Deserialize, Debug)]
struct Port {
	name: String,
	key: String,
	color: String,
	repository: Repository,
	categories: Vec<Category>,
	platform: Vec<Platform>
}

#[derive(Serialize, Deserialize, Debug)]
struct Ports {
	ports: Vec<Port>,
	collaborators: Vec<Collaborator>
}

fn extract_repository_location_from_url(repo_url: &str) -> &str {
	repo_url.split("/").last().unwrap()
}

// FIXME: Use https://crates.io/crates/auth_git2
fn clone_repository(repo_url: &str, to: &str) -> GitRepository {
	let home = std::env::var("HOME").unwrap();

	// SAFETY: We are not multithreaded
	unsafe {
		std::env::set_var("HOME", "/tmp/cli-home");
	}
	
	let repo = match GitRepository::clone(&repo_url, to) {
		Ok(repo) => repo,
		Err(e) => panic!("failed to clone: {}", e),
	};

	// SAFETY: We are not multithreaded
	unsafe {
		std::env::set_var("HOME", home);
	}
	
	repo
}

fn review_command(repo_url: String, skip_clone: bool) {
	let location = extract_repository_location_from_url(&repo_url);
	
	let repo = if skip_clone {
		GitRepository::open(location).unwrap()
	} else {
		info!("Cloning {repo_url} to {location}");
		
		clone_repository(&repo_url, location)
	};
	
	let contracts: Vec<Box<dyn ReviewContract>> = vec![
		Box::new(NoGitKeepContract),
		Box::new(ReadmeContract),
		Box::new(AssetsContract),
		Box::new(LicenseContract)
	];

	for contract in contracts {
		let name = contract.name();
		info!("Testing contract '{name}'");
		
		match contract.test(&repo) {
			ContractResult::Fail { msg } => {
				error!("Contract '{name}' failed:\n{msg}");
			},
			ContractResult::Warn { msg } => {
				warn!("Contract '{name}' warned: {msg}");
			},
			ContractResult::Pass => {
				info!("Contract '{name}' passed");
			}
		}
	}
}

fn main() {
	let env = env_logger::Env::default()
		.filter_or("RUST_LOG", "info");

	env_logger::init_from_env(env);

	let yml_src = include_str!("../ports.yml");
	let ports_yml = serde_yml::from_str::<Ports>(yml_src).unwrap();
	
	let args = Args::parse();

	match args.command {
		Command::Review { url, skip_clone } => {
			review_command(url, skip_clone);
		}
	}
}
