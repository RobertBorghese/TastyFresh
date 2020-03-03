/**********************************************************
 * --- Header Context ---
 *
 * Keeps track of the header files that need to be added
 * depending on the usage of certain classes and functions.
 **********************************************************/

use crate::declaration_parser::include_declaration::IncludeType;

pub struct HeaderContext {
	pub headers: Vec<Header>,
}

impl HeaderContext {
	pub fn new() -> HeaderContext {
		return HeaderContext {
			headers: Vec::new()
		};
	}

	pub fn add_header(&mut self, path: String, is_system: bool) {
		if !self.contains(&path) {
			self.headers.push(Header::new(path, if is_system {
				IncludeType::Header
			} else {
				IncludeType::Local
			}));
		}
	}

	pub fn contains(&self, path: &str) -> bool {
		for h in &self.headers {
			if h.path == path {
				return true;
			}
		}
		return false;
	}

	pub fn len(&self) -> usize {
		return self.headers.len();
	}

	pub fn is_empty(&self) -> bool {
		return self.len() <= 0;
	}
}

pub struct Header {
	pub path: String,
	pub inc_type: IncludeType
}

impl Header {
	pub fn new(path: String, inc_type: IncludeType) -> Header {
		return Header {
			path: path,
			inc_type: inc_type
		};
	}
}
