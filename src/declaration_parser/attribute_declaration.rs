/**********************************************************
 * --- Attribute Declaration ---
 *
 * Represents a attribute declaration prior to being parsed.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_ascii,
	declare_parse_expr_until_either_char
};

use crate::context_management::global_context::GlobalContext;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

use either::*;

type AttributeDeclarationResult = DeclarationResult<AttributeDeclaration>;

#[derive(Clone)]
pub struct AttributeDeclaration {
	pub name: String,
	pub parameters: Option<Vec<Either<(usize, usize),String>>>,
	pub line: usize
}

impl Declaration<AttributeDeclaration> for AttributeDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of attribute";
	}
}

impl CPPTranspiler for AttributeDeclaration {
	fn to_cpp(&self) -> String {
		return "".to_string();
	}
}

impl AttributeDeclaration {
	pub fn new(parser: &mut Parser, store_params: bool) -> AttributeDeclarationResult {
		let initial_line = parser.line;

		let mut next_char = parser.get_curr();
		if next_char != '@' {
			return Self::unexpected_character(parser.index);
		}
		parser.increment();

		// Parse Var Style
		let mut attribute_name = "".to_string();
		declare_parse_ascii!(attribute_name, parser);

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		let mut parameters = None;
		next_char = parser.get_curr();
		if next_char == '(' {
			parser.increment();
			let mut params = Vec::new();
			loop {
				let start = parser.index;
				let mut result = ' ';
				declare_parse_expr_until_either_char!(',', ')', result, parser);
				if result == ')' {
					params.push(if store_params {
						Right(parser.content[start..parser.index].to_string())
					} else {
						Left((start, parser.index))
					});
					break;
				} else if result == ',' {
					params.push(if store_params {
						Right(parser.content[start..parser.index].to_string())
					} else {
						Left((start, parser.index))
					});
					parser.increment();
				} else {
					return Self::out_of_space(parser.index);
				}
			}
			parameters = Some(params);
			parser.increment();
		}

		return AttributeDeclarationResult::Ok(AttributeDeclaration {
			name: attribute_name,
			parameters: parameters,
			line: initial_line
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_attribute_declaration(&parser.content, parser.index);
	}

	pub fn is_attribute_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("@");
	}

	pub fn params_length(&self) -> usize {
		return if self.parameters.is_some() { self.parameters.as_ref().unwrap().len() } else { 0 };
	}

	pub fn get_param(&self, index: usize, content: &str) -> String {
		let param = &self.parameters.as_ref().unwrap()[index];
		if param.is_left() {
			let p = param.as_ref().left().unwrap();
			return content[p.0..p.1].to_string();
		} else {
			return param.as_ref().right().unwrap().clone();
		};
	}

	pub fn flatten_attributes(attributes: &mut Vec<AttributeDeclaration>, global_context: &GlobalContext, content: &str) {
		let mut new_attributes = Vec::new();
		for a in attributes.iter() {
			let attr_cls = global_context.find_attribute(&a.name);
			if attr_cls.is_some() {
				let new_attr = attr_cls.unwrap().output_new_attributes(a, content);
				for at in new_attr {
					new_attributes.push(at);
				}
			}
		}
		for n in new_attributes {
			attributes.push(n);
		}
	}
}
