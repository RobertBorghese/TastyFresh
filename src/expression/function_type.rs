/**********************************************************
 * --- Function Type ---
 *
 * The enum defined here represents the different types
 * of quirks available for functions.
 **********************************************************/

lazy_static! {
	pub static ref FUNCTION_STYLES: Vec<&'static str> = vec!("static", "virtual", "inline", "meta");
}

pub enum FunStyle {
	Unknown,
	Static,
	Virtual,
	Inline,
	Meta
}

impl FunStyle {
	pub fn new(name: &str) -> FunStyle {
		return match name {
			"static" => FunStyle::Static,
			"virtual" => FunStyle::Virtual,
			"inline" => FunStyle::Inline,
			"meta" => FunStyle::Meta,
			_ => FunStyle::Unknown
		}
	}

	pub fn styles() -> &'static Vec<&'static str> {
		return &FUNCTION_STYLES;
	}

	pub fn get_name(&self) -> &str {
		return match self {
			FunStyle::Unknown => "",
			FunStyle::Static => "static",
			FunStyle::Virtual => "virtual",
			FunStyle::Inline => "inline",
			FunStyle::Meta => "meta"
		}
	}

	pub fn is_unknown(&self) -> bool {
		return match self {
			FunStyle::Unknown => true,
			_ => false
		}
	}

	pub fn class_only(&self) -> bool {
		return match self {
			FunStyle::Virtual => true,
			_ => false
		}
	}

	pub fn module_only(&self) -> bool {
		return match self {
			FunStyle::Meta => true,
			_ => false
		}
	}
}
