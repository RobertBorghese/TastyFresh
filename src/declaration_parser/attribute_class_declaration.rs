/**********************************************************
 * --- Attribute Class Declaration ---
 *
 * Represents the a class (or class-like) declaration.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_ascii,
	declare_parse_required_next_char
};

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::declaration_parser::attribute_declaration::AttributeDeclaration;

use either::*;

type AttributeClassDeclarationResult = DeclarationResult<AttributeClassDeclaration>;

pub struct AttributeClassDeclaration {
	pub name: String,
	pub arguments: Vec<(String,bool)>,
	pub attributes: Vec<AttributeDeclaration>
}

impl Declaration<AttributeClassDeclaration> for AttributeClassDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl AttributeClassDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str) -> AttributeClassDeclarationResult {

		// Parse Attribute Word
		let mut attribute_keyword = "".to_string();
		declare_parse_ascii!(attribute_keyword, parser);
		if attribute_keyword != "attribute" {
			return AttributeClassDeclarationResult::Err("Unexpected Keyword", "\"attribute\" keyword expected", parser.index - attribute_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		// Parse Attribute Name
		let mut attribute_name = "".to_string();
		declare_parse_required_ascii!(attribute_name, "Attribute Name Missing", "attribute name missing", parser);

		declare_parse_whitespace!(parser);

		let mut attribute_params = Vec::new();
		if parser.get_curr() == '(' {
			parser.increment();
			loop {
				// Parse Attribute Param
				declare_parse_whitespace!(parser);
				let trim_param = parser.get_curr() == '[';
				if trim_param {
					parser.increment();
				}
				let mut param_name = "".to_string();
				declare_parse_required_ascii!(param_name, "Attribute Param Name Missing", "attribute parameter name missing", parser);
				attribute_params.push((param_name, trim_param));
				if trim_param {
					let mut next_char = ' ';
					declare_parse_required_next_char!(']', next_char, parser);
				}
				declare_parse_whitespace!(parser);
				if parser.get_curr() == ')' {
					parser.increment();
					break;
				} else {
					let mut next_char = ' ';
					declare_parse_required_next_char!(',', next_char, parser);
				}
			}
		}

		declare_parse_whitespace!(parser);

		let mut next_char = ' ';
		declare_parse_required_next_char!('{', next_char, parser);

		let mut attributes = Vec::new();

		while !parser.out_of_space {
			parser.parse_whitespace();

			let initial_index = parser.index;

			if AttributeDeclaration::is_declaration(parser) {
				let result = AttributeDeclaration::new(parser, true);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					attributes.push(result.unwrap_and_move());
				}
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

		return AttributeClassDeclarationResult::Ok(AttributeClassDeclaration {
			name: attribute_name,
			arguments: attribute_params,
			attributes: attributes
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_attribute_class_declaration(&parser.content, parser.index);
	}

	pub fn is_attribute_class_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("attribute ");
	}

	pub fn output_new_attributes(&self, attribute: &AttributeDeclaration, content: &str) -> Vec<AttributeDeclaration> {
		let mut new_args = Vec::new();
		let cls_args = &self.arguments;
		for i in 0..attribute.params_length() {
			if i < cls_args.len() {
				new_args.push(vec![attribute.get_param(i, content)]);
			} else if !new_args.is_empty() {
				new_args.last_mut().unwrap().push(attribute.get_param(i, content));
			}
		}

		let mut result = Vec::new();
		for new_attr in &self.attributes {
			let mut attr_params = Vec::new();
			let mut a = new_attr.clone();
			for i in 0..a.params_length() {
				let mut p = a.get_param(i, content);
				for j in 0..cls_args.len() {
					if j >= new_args.len() { break; }
					let should_trim = !cls_args[j].1;
					if p == cls_args[j].0 {
						for arg in &new_args[j] {
							attr_params.push(Right(if should_trim { arg.trim().to_string() } else { arg.to_string() }));
						}
					} else {
						let search_str = format!("[{}]", cls_args[j].0);
						if p.contains(&search_str) {
							let mut search_str_result = Vec::new();
							for arg in &new_args[j] {
								search_str_result.push(arg.to_string());
							}
							let final_content = search_str_result.join(",");
							p = p.replace(search_str.as_str(), if should_trim { final_content.trim() } else { final_content.as_str() });
						}
					}
				}
				attr_params.push(Right(p.to_string()));
			}
			a.parameters = Some(attr_params);
			result.push(a);
		}

		return result;
	}
}
