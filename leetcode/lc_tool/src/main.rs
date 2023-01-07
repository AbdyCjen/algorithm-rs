mod problem;

use clap::{command, Args, Parser};
use std::{fs, io::Write, path::Path};

#[derive(Parser)]
#[command(author, version, about)]
enum Command {
	Random,
	Pick(ProbelmId),
	Daily,
}

#[derive(Args)]
struct ProbelmId {
	id: u32,
}

/// main() helps to generate the submission template .rs
fn main() -> anyhow::Result<()> {
	let cmd = Command::parse();
	let solved_ids = get_solved_ids()?;
	let id = match cmd {
		Command::Daily => problem::get_daily_id()?,
		Command::Pick(p) => p.id,
		Command::Random => generate_random_id(&solved_ids),
	};

	anyhow::ensure!(
		!solved_ids.contains(&id),
		"problem-{id} is already solved/initialized"
	);

	let problem = problem::get_problem(id)?;

	let code = problem
		.code_definition
		.iter()
		.find(|&d| d.value == "rust")
		.ok_or_else(|| anyhow::format_err!("Problem {} has no rust version.", &id))?;

	let file_name = format!("n{:04}_{}", id, problem.title_slug.replace('-', "_"));
	let file_path = Path::new("./src").join(format!("{}.rs", file_name));
	anyhow::ensure!(!file_path.exists(), "problem-{id} already  initialized");

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

	std::fs::write(file_path, source.as_bytes())?;

	let mut lib_file = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open("./src/lib.rs")?;
	writeln!(lib_file, "mod {};", file_name)?;
	Ok(())
}

fn generate_random_id(except_ids: &[u32]) -> u32 {
	use rand::Rng;
	let mut rng = rand::thread_rng();
	loop {
		const MAX_QUESTION_ID: u32 = 2523;
		let res: u32 = rng.gen_range(1..=MAX_QUESTION_ID);
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

fn get_solved_ids() -> anyhow::Result<Vec<u32>> {
	let mut solved_ids = Vec::new();
	for entry in fs::read_dir("./src")? {
		let file_name = entry?.file_name().into_string().unwrap();
		if !file_name.starts_with('n') {
			continue;
		}
		let id = file_name[1..5].parse::<u32>()?;
		solved_ids.push(id);
	}
	Ok(solved_ids)
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
		.replace('\n', "\n * ")
		.replace("\r\n", "\n")
		.replace('\t', "    ")
}
