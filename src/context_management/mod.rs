/**********************************************************
 * --- Context Management ---
 *
 * Manages the contextual information while parsing
 * the Tasty Fresh code.
 **********************************************************/

pub mod position;

use position::Position;

use num::*;

pub fn print_code_error(title: &str, message: &str, position: &Position) {
	let mut output = String::from("");

	// title
	output += "==============================\n";
	output += format!("{} - {}\n", title, position.file).as_str();
	output += "==============================\n";

	// contents
	let file_content = "8934293048940230984238423574890589345893405893405835803450";
	let file_chars: Vec<char> = file_content.chars().collect();
	let mut line_content = "".to_string();

	let mut line = position.line.unwrap_or(1) - 1;
	let mut start = position.start;
	let mut end = position.end.unwrap_or(position.start + 1);

	if position.line.is_none() {
		let mut line_start = 0;
		let mut i = 0;
		loop {
			if i >= file_chars.len() {
				break;
			}
			if file_chars[i] == '\n' {
				if i >= end {
					break;
				}
				line += 1;
				line_start = i;
			}
			if i >= start {
				if i == start {
					start -= line_start;
					end -= line_start;
				}
			}
			i += 1;
		}
	}

	let mut i = 0;
	let mut temp_line = 0;
	loop {
		if i >= file_chars.len() {
			break;
		}
		if file_chars[i] == '\n' {
			temp_line += 1;
		}
		if temp_line == line {
			line_content.push(file_chars[i]);
		} else if temp_line > line {
			break;
		}
		i += 1;
	}

	let line_text = (line + 1).to_string();
	let line_digits = line_text.len();
	let spaces = create_spacing(line_digits);

	output += format!("{} |\n", spaces).as_str();
	output += format!("{} |    {}\n", line_text, line_content).as_str();
	output += format!("{} |    {}{} {}\n", spaces, create_spacing(start), repeat_char(b'^', end - start), message).as_str();

	println!("{}\n\n", output);
}

fn create_spacing(count: usize) -> String {
	return repeat_char(b' ', count);
}

fn repeat_char(c: u8, count: usize) -> String {
	return String::from_utf8(vec![c; count]).unwrap_or("".to_string());
}

