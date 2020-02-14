/**********************************************************
 * --- Position ---
 *
 * Represents a position in the Tasty Fresh code.
 * Includes a file, line number, start index, and end index.
 **********************************************************/

pub struct Position {
	pub file: String,
	pub line: Option<usize>,
	pub start: usize,
	pub end: Option<usize>
}

impl Position {
	pub fn new(file: String, line: Option<usize>, start: usize, end: Option<usize>) -> Position {
		return Position {
			file: file,
			line: line,
			start: start,
			end: end
		};
	}
}
