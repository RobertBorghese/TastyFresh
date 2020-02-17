/**********************************************************
 * --- File System ---
 *
 * Functions related to reading and writing the files
 * required for the Tasty Fresh compiler.
 **********************************************************/

use std::path::{ Path, PathBuf };

use path_slash::PathExt;

/// Returns a `Vec` of all file names with the specified file extension in the specified directory.
/// This recursively retrieves all files in sub-directories as well.
///
/// # Arguments
///
/// * `dir` - The directory, either relative or absolute, to search for the files.
/// * `file_extension` - The desired file extension to search for.
///
/// # Return
///
/// An `Option<Vec<String>>` containing the file names. If the directory doesn't exist, `None` is returned.
pub fn get_all_files(dir: &str, file_extension: &str) -> Option<Vec<String>> {
	let mut result = Vec::new();
	let path = Path::new(dir);
	if !path.exists() || !path.is_dir() {
		return None;
	} else if dir.is_empty() || file_extension.is_empty() {
		return None;
	} else {
		for entry in glob::glob(format!("{}/**/*.{}", dir, file_extension).as_str()).expect("Could not parse input dir.") {
			match entry {
				Ok(path) => match path.as_path().to_slash() {
					Some(path) => result.push(path),
					None => ()
				},
				Err(_) => ()
			}
		}
	}
	return Some(result);
}

/// Returns a `Vec` of all `.tasty` files in the specified directory.
///
/// # Arguments
///
/// * `dir` - The directory, either relative or absolute, to search for the files.
///
/// # Return
///
/// An `Option<Vec<String>>` containing the file names. If the directory doesn't exist, `None` is returned.
pub fn get_all_tasty_files(dir: &str) -> Option<Vec<String>> {
	return get_all_files(dir, "tasty");
}
