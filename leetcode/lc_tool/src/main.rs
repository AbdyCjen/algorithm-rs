#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod problem;

use std::{fs, io::Write, path::Path};
use clap::clap_app;

/// main() helps to generate the submission template .rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let matches = clap_app! { lc_tool =>
		(version: env!("CARGO_PKG_VERSION"))
		(author : env!("CARGO_PKG_AUTHORS"))
		(about : env!("CARGO_PKG_DESCRIPTION"))
		(@setting DisableHelpSubcommand)
		(@setting VersionlessSubcommands)
		(@arg id: -i +takes_value +required "question id, if not set, pick a rand one")
		(@arg random: -r !takes_value !required "random pick flag, if set, pick a random unsolved question")
	}
	.get_matches();
	let solved_ids = get_solved_ids();
	let id = if matches.is_present("random") {
		generate_random_id(&solved_ids)
	} else {
		matches.value_of("id").unwrap().parse::<u32>().unwrap()
	};
	if solved_ids.contains(&id) {
		println!("the problem is already solved");
		return Ok(());
	}

	let problem = problem::get_problem(id).unwrap_or_else(|| {
		panic!(
			"Error: failed to get problem #{} \
             (The problem may be paid-only or may not be exist).", id)});

	let code = problem.code_definition.iter().find(|&d| d.value == "rust")
		.unwrap_or_else(|| panic!("Problem {} has no rust version.", &id));

	let file_name = format!("n{:04}_{}", id, problem.title_slug.replace("-", "_"));
	let file_path = Path::new("./src").join(format!("{}.rs", file_name));
	if file_path.exists() {
		panic!("problem already initialized");
	}

	let template = include_str!("../template.rs");
	let source = template
		.replace("__PROBLEM_TITLE__", &problem.title)
		.replace("__PROBLEM_DESC__", &build_desc(&problem.content))
		.replace("__PROBLEM_DEFAULT_CODE__", &code.default_code)
		.replace("__PROBLEM_ID__", &format!("{}", id))
		.replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
		.replace(
			"__EXTRA_TEST_USE__",
			&parse_extra_test_use(&code.default_code),
		);

	let mut file = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(&file_path)
		.unwrap();

	file.write_all(source.as_bytes()).unwrap();
	drop(file);

	let mut lib_file = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open("./src/lib.rs")
		.unwrap();
	writeln!(lib_file, "mod {};", file_name)?;
	Ok(())
}

fn generate_random_id(except_ids: &[u32]) -> u32 {
	use rand::Rng;
	let mut rng = rand::thread_rng();
	loop {
		const MAX_QUESTION_ID: u32 = 1883;
		let res: u32 = rng.gen_range(1, MAX_QUESTION_ID);
		if !except_ids.contains(&res) {
			return res;
		}
		println!(
			"Generate a random num ({}), but it is invalid (the problem may have been solved \
			 or may have no rust version). Regenerate..",
			res
		);
	}
}

fn get_solved_ids() -> Vec<u32> {
	let paths = fs::read_dir("./src").unwrap();
	let mut solved_ids = Vec::new();

	for path in paths {
		let path = path.unwrap().path();
		let s = path.to_str().unwrap();
		if !s.starts_with('n') {
			continue;
		}
		let id = &s[7..11];
		let id = id.parse::<u32>().unwrap();
		solved_ids.push(id);
	}
	solved_ids
}

fn parse_extra_use(code: &str) -> String {
	let mut extra_use_line = String::new();
	// a linked-list problem
	// put to_tree, to_list in test block instead, or clippy will warn unused use;
	if code.contains("pub struct ListNode") {
		extra_use_line.push_str("\nuse super::util::linked_list::ListNode;")
	}
	if code.contains("pub struct TreeNode") {
		extra_use_line.push_str("\nuse super::util::tree::TreeNode;")
	}
	if code.contains("pub struct Point") {
		extra_use_line.push_str("\nuse super::util::point::Point;")
	}
	extra_use_line
}

fn parse_extra_test_use(code: &str) -> String {
	let mut extra_test_use_line = String::new();
	// a linked-list problem
	if code.contains("pub struct ListNode") {
		extra_test_use_line.push_str("use super::{super::util::linked_list::to_list,*};")
	}
	if code.contains("pub struct TreeNode") {
		extra_test_use_line.push_str("use super::{super::util::tree::to_tree, *};")
	} else {
		extra_test_use_line.push_str("use super::*;")
	}
	extra_test_use_line
}

fn build_desc(content: &str) -> String {
	// TODO: fix this shit
	content
		.replace("<strong>", "")
		.replace("</strong>", "")
		.replace("<em>", "")
		.replace("</em>", "")
		.replace("</p>", "")
		.replace("<p>", "")
		.replace("<b>", "")
		.replace("</b>", "")
		.replace("<pre>", "")
		.replace("</pre>", "")
		.replace("<ul>", "")
		.replace("</ul>", "")
		.replace("<li>", "")
		.replace("</li>", "")
		.replace("<code>", "")
		.replace("</code>", "")
		.replace("<i>", "")
		.replace("</i>", "")
		.replace("<sub>", "")
		.replace("</sub>", "")
		.replace("</sup>", "")
		.replace("<sup>", "^")
		.replace("&nbsp;", " ")
		.replace("&gt;", ">")
		.replace("&lt;", "<")
		.replace("&quot;", "\"")
		.replace("&minus;", "-")
		.replace("&#39;", "'")
		.replace("\n\n", "\n")
		.replace("\n", "\n * ")
		.replace("\r\n", "\n")
		.replace("\t", "    ")
}
