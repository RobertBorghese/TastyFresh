/**********************************************************
 * --- Context Management ---
 *
 * Manages the contextual information while parsing
 * the Tasty Fresh code.
 **********************************************************/

pub mod position;

use position::Position;

use num::*;

pub fn print_code_error(title: String, message: String, position: Position) {
	let mut output = String::from("");

	// title
	output += "==========\n";
	output += format!("{} - {}\n", title, position.file).as_str();
	output += "==========\n";

	// contents
	let line = position.line;
	let line_digits = line.to_string().len();
	let spaces = create_spacing(line_digits);

	// TODO: get actual line contents
	let line_content = "89342930489402309842384230";

	output += format!("{} |\n", spaces).as_str();
	output += format!("{} |    {}\n", line.to_string(), line_content).as_str();
	output += format!("{} |    {}{}{}\n", spaces, create_spacing(position.start), repeat_char(b'^', position.end.unwrap_or(position.start) - position.start), message).as_str();

	println!("{}\n\n", output);
}

fn create_spacing(count: usize) -> String {
	return repeat_char(b' ', count);
}

fn repeat_char(c: u8, count: usize) -> String {
	return String::from_utf8(vec![c; count]).unwrap_or("".to_string());
}

