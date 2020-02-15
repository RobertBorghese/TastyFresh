/**********************************************************
 * --- Attribute Type ---
 *
 * The structs and enums defined here represent functions
 * within Tasty Fresh.
 **********************************************************/

use std::result::Result;

lazy_static! {
	pub static ref SYSTEM_ATTRIBUTES: Vec<&'static str> = vec!("preference", "append", "prepend", "include");
}

static ref LOCATION_ERROR = "location should be \"header\", \"source\", or \"both\"";

/// Stores the attribute type.
pub enum Attribute {
	Custom(Vec<String>),
	Preference(PreferenceType, PreferenceType),
	Append(String, AttributeLocation),
	Prepend(String, AttributeLocation),
	Include(String, AttributeLocation)
}

impl Attribute {
	pub fn new(name: &str, params: Vec<String>) -> AttributeResult {
		return match name {
			"preference" => new_preference(params),
			"append" => new_append(params),
			"prepend" => new_prepend(params),
			"include" => new_include(params)
		}
	}

	fn new_preference(params: Vec<String>) -> AttributeResult {
		if params.len() < 2 || params.len() > 2 { return AttributeResult::Err("@preference takes only 2 parameters", AttributeParamIndex::All); }
		return Preference(PreferenceType::new(params[0]), PreferenceType::new(params[1]));
	}

	fn new_append(params: Vec<String>) -> AttributeResult {
		return match params.len() {
			0 => AttributeResult::Err("@append requires content", AttributeParamIndex::All),
			1 => AttributeResult::Ok(Attribute::Append(params[0], AttributeLocation::Header)),
			2 => {
				let loc = AttributeLocation::new(params[1]);
				if loc.is_none() {
					AttributeResult::Err(LOCATION_ERROR, AttributeParamIndex::One(1))
				} else {
					AttributeResult::Ok(Attribute::Append(params[0], loc.unwrap()))
				}
			},
			_ => AttributeResult::Err("@append takes only 2 parameters", AttributeParamIndex::All);
		}
	}

	fn new_prepend(params: Vec<String>) -> AttributeResult {
		return match params.len() {
			0 => AttributeResult::Err("@prepend requires content", AttributeParamIndex::All),
			1 => AttributeResult::Ok(Attribute::Prepend(params[0], AttributeLocation::Header)),
			2 => {
				let loc = AttributeLocation::new(params[1]);
				if loc.is_none() {
					AttributeResult::Err(LOCATION_ERROR, AttributeParamIndex::One(1))
				} else {
					AttributeResult::Ok(Attribute::Prepend(params[0], loc.unwrap()))
				}
			},
			_ => AttributeResult::Err("@prepend takes only 2 parameters", AttributeParamIndex::All);
		}
	}

	fn new_include(params: Vec<String>) -> AttributeResult {
		return match params.len() {
			0 => AttributeResult::Err("@include requires header path", AttributeParamIndex::All),
			1 => AttributeResult::Ok(Attribute::Include(params[0], AttributeLocation::Header)),
			2 => {
				let loc = AttributeLocation::new(params[1]);
				if loc.is_none() {
					AttributeResult::Err(LOCATION_ERROR, AttributeParamIndex::One(1))
				} else {
					AttributeResult::Ok(Attribute::Include(params[0], loc.unwrap()))
				}
			},
			_ => AttributeResult::Err("@include takes only 2 parameters", AttributeParamIndex::All);
		}
	}

	pub fn attributes() -> &'static Vec<&'static str> {
		return &SYSTEM_ATTRIBUTES;
	}

	pub fn get_name(&self) -> &str {
		return match self {
			Quirk::Unknown => "",
			Quirk::Static => "static",
			Quirk::Virtual => "virtual",
			Quirk::Inline => "inline",
			Quirk::Meta => "meta"
		}
	}

	pub fn is_unknown(&self) -> bool {
		return match self {
			Quirk::Unknown => true,
			_ => false
		}
	}
}

/// Stores the preference type for the `@preference` attribute. 
pub enum PreferenceType {
	Default,
	None,
	Function(String)
}

impl PreferenceType {
	pub fn new(param: String) -> PreferenceType {
		return match param {
			"default" => PreferenceType::Default,
			"none" => PreferenceType::None,
			func_name => PreferenceType::Function(func_name)
		}
	}
}

/// Stores the location of the `@append`, `@prepend`, or `@include` attributes.
pub enum AttributeLocation {
	Header,
	Source,
	Both
}

impl AttributeLocation {
	pub fn new(param: String) -> Option<AttributeLocation> {
		return match param {
			"header" => Some(AttributeLocation::Header),
			"source" => Some(AttributeLocation::Source),
			"both" => Some(AttributeLocation::Both),
			_ => None
		}
	}
}

/// Stores the result of a parsed attribute.
pub enum AttributeResult {
	Ok(Attribute),
	Err(&'static str, AttributeParamIndex)
}

pub enum AttributeParamIndex {
	All,
	One(usize)
}
