/**********************************************************
 * --- Function Type ---
 *
 * The enum defined here represents the different types
 * of quirks available for functions.
 **********************************************************/

lazy_static! {
	pub static ref FUNCTION_STYLES: Vec<&'static str> = vec!("static", "extern", "virtual", "inline", "meta", "const", "override");
}

#[derive(Clone, PartialEq)]
pub enum FunStyle {
	Unknown,
	Static,
	Extern,
	Virtual,
	Inline,
	Meta,
	Const,
	Override
}

impl FunStyle {
	pub fn new(name: &str) -> FunStyle {
		return match name {
			"static" => FunStyle::Static,
			"extern" => FunStyle::Extern,
			"virtual" => FunStyle::Virtual,
			"inline" => FunStyle::Inline,
			"meta" => FunStyle::Meta,
			"const" => FunStyle::Const,
			"override" => FunStyle::Override,
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
			FunStyle::Extern => "extern",
			FunStyle::Virtual => "virtual",
			FunStyle::Inline => "inline",
			FunStyle::Meta => "meta",
			FunStyle::Const => "const",
			FunStyle::Override => "override"
		}
	}

	pub fn is_extern(&self) -> bool {
		return match self {
			FunStyle::Extern => true,
			_ => false
		}
	}

	pub fn is_virtual(&self) -> bool {
		return match self {
			FunStyle::Virtual => true,
			_ => false
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
			FunStyle::Extern => true,
			_ => false
		}
	}

	pub fn class_exportable(&self) -> bool {
		return match self {
			FunStyle::Virtual => true,
			FunStyle::Inline => true,
			FunStyle::Static => true,
			FunStyle::Override => true,
			FunStyle::Const => true,
			_ => false
		}
	}

	pub fn module_exportable(&self) -> bool {
		return match self {
			FunStyle::Extern => true,
			FunStyle::Inline => true,
			FunStyle::Static => true,
			_ => false
		}
	}
}
