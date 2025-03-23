use git2::{Repository};
use std::fs;
use std::path::PathBuf;

pub enum ContractResult {
	Fail { msg: String },
	Warn { msg: String },
	Pass
}

pub trait ReviewContract {
	fn name(&self) -> &str; 
	fn test(&self, repo: &Repository) -> ContractResult;
}

pub struct NoGitKeepContract;
impl ReviewContract for NoGitKeepContract {
	fn name(&self) -> &str {
		"No .gitkeep files"
	}
	
	fn test(&self, repo: &Repository) -> ContractResult {
		let index = repo.index().unwrap();
		for entry in index.iter() {
			let p = String::from_utf8(entry.path).unwrap();
			
			if p.contains(".gitkeep") {
				return ContractResult::Fail { msg: format!("path at {p}") };
			}
		}
		
		ContractResult::Pass
	}
}

pub struct ReadmeContract;
impl ReviewContract for ReadmeContract {
	fn name(&self) -> &str {
		"README is correct"
	}

	fn test(&self, repo: &Repository) -> ContractResult {
		let to_check = [
			"https://github.com/catppuccin/template/stargazers",
			"https://github.com/catppuccin/template/issues",
			"https://github.com/catppuccin/template/contributors",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/latte.webp",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/frappe.webp",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/macchiato.webp",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/mocha.webp",
			"- [Human](https://github.com/catppuccin)"
		];

		for s in to_check {
			let path = repo.workdir().unwrap().join("README.md");
			let readme = fs::read_to_string(path).unwrap();
			
			if readme.contains(s) {
				return ContractResult::Fail { msg: format!("README contains '{s}'") }
			}
		}

		ContractResult::Pass
	}
}

pub struct LicenseContract;
impl ReviewContract for LicenseContract {
	fn name(&self) -> &str {
		"LICENSE is correct"
	}

	fn test(&self, repo: &Repository) -> ContractResult {
		let to_check = [
			"https://github.com/catppuccin/template/stargazers",
			"https://github.com/catppuccin/template/issues",
			"https://github.com/catppuccin/template/contributors",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/latte.webp",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/frappe.webp",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/macchiato.webp",
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/mocha.webp",
			"- [Human](https://github.com/catppuccin)"
		];

		let path = repo.workdir().unwrap().join("LICENSE");
		let license = fs::read_to_string(path).unwrap();
		let needle = "Copyright (c) 2021 Catppuccin";
		
		if !license.contains(needle) {
			return ContractResult::Warn { msg: format!("LICENSE header is wrong, expected {needle}") }
		}

		ContractResult::Pass
	}
}

pub struct AssetsContract;
impl ReviewContract for AssetsContract {
	fn name(&self) -> &str {
		"Assets are correct"
	}

	fn test(&self, repo: &Repository) -> ContractResult {
		let paths = [
			PathBuf::from("assets/mocha.webp"),
			PathBuf::from("assets/latte.webp"),
			PathBuf::from("assets/macchiato.webp"),
			PathBuf::from("assets/frappe.webp")
		];

		for p in paths.iter() {
			if repo.status_file(p).is_err() {
				return ContractResult::Fail { msg: format!("path '{}' is not valid", p.display()) }
			}
		}

		ContractResult::Pass
	}
}
