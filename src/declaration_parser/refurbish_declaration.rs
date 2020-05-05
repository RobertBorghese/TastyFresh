/**********************************************************
 * --- Refurbish Declaration ---
 *
 * Represents the a refurbish declaration.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_next_char,
	declare_parse_type
};

use crate::config_management::operator_data::OperatorDataStructure;

use crate::expression::variable_type::Type;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::module_declaration::DeclarationType;
use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use crate::declaration_parser::attributes::Attributes;

use regex::Regex;

lazy_static! {
	pub static ref REFURBISH_REGEX: Regex = Regex::new(r"^\b(?:refurbish)\b").unwrap();
}

type RefurbishDeclarationResult = DeclarationResult<RefurbishDeclaration>;

#[derive(Clone)]
pub struct RefurbishDeclaration {
	pub refurbish_type: Type,
	pub declarations: Vec<DeclarationType>
}

impl Declaration<RefurbishDeclaration> for RefurbishDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl RefurbishDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str, operator_data: &OperatorDataStructure) -> RefurbishDeclarationResult {

		let mut refurbish_keyword = "".to_string();
		declare_parse_ascii!(refurbish_keyword, parser);
		if refurbish_keyword != "refurbish" {
			return RefurbishDeclarationResult::Err("Unexpected Keyword", "\"refurbish\" keyword expected", parser.index - refurbish_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		let refurbish_type: Type;
		declare_parse_type!(refurbish_type, parser);

		declare_parse_whitespace!(parser);

		let mut next_char = ' ';
		declare_parse_required_next_char!('{', next_char, parser);

		let mut declarations = Vec::new();
		let mut attributes = Vec::new();

		while !parser.out_of_space {
			parser.parse_whitespace();

			let initial_index = parser.index;

			if AttributeDeclaration::is_declaration(parser) {
				let result = AttributeDeclaration::new(parser, false);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					attributes.push(result.unwrap_and_move());
				}
				continue;
			}

			if FunctionDeclaration::is_declaration(parser) {
				let result = FunctionDeclaration::new(parser, FunctionDeclarationType::ClassLevel, Some(operator_data));
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::Function(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					})));	
				}
				attributes.clear();
				parser.increment();
				continue;
			}

			if parser.get_curr() == '}' {
				break;
			}

			if !parser.out_of_space {
				parser.increment();
			}

			if parser.index == initial_index {
				break;
			}
		}

		return RefurbishDeclarationResult::Ok(RefurbishDeclaration {
			refurbish_type: refurbish_type,
			declarations: declarations
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_refurbish_declaration(&parser.content, parser.index);
	}

	pub fn is_refurbish_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return REFURBISH_REGEX.is_match(declare);
	}

	pub fn make_name(&self) -> String {
		let re = Regex::new(r"(?:\.|::|<|>|,|\s)").unwrap();
		return re.replace_all(&self.refurbish_type.to_cpp(false), "_").to_string();
	}
}