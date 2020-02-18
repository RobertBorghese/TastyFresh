/**********************************************************
 * --- Function Declaration ---
 *
 * Represents a function declaration prior to being parsed.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_ascii,
	declare_parse_required_next_char,
	declare_parse_expr_until_next_char,
	declare_parse_expr_until_either_char,
	declare_parse_type,
	delcare_increment
};

use crate::expression::variable_type::{ VariableType, Type, VarStyle, VarProps };

use crate::expression::function_type::FunStyle;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

type FunctionDeclarationResult = DeclarationResult<FunctionDeclaration>;

pub struct FunctionDeclaration {
	pub name: String,
	pub props: Vec<FunStyle>,
	pub parameters: Vec<(VariableType, String, usize, usize)>,
	pub return_type: Option<VariableType>,
	pub line: usize,
	pub start_index: usize,
	pub end_index: usize
}

pub enum FunctionDeclarationType {
	ModuleLevel,
	ClassLevel,
	Assumption
}

impl Declaration<FunctionDeclaration> for FunctionDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl FunctionDeclaration {
	pub fn new(parser: &mut Parser, declare_type: FunctionDeclarationType) -> FunctionDeclarationResult {
		let initial_line = parser.line;

		let mut func_props = Vec::new();

		// Parse Function Properties and Style
		let mut successfully_parsed = false;
		let mut name = "".to_string();
		while Self::is_func_declaration(parser.content, parser.index) {
			name = "".to_string();
			declare_parse_ascii!(name, parser);
			if FunStyle::styles().contains(&name.as_str()) {
				let style = FunStyle::new(name.as_str());
				match declare_type {
					FunctionDeclarationType::ModuleLevel => {
						if style.class_only() {
							return FunctionDeclarationResult::Err("Style Disallowed", "style only allowed in class functions", parser.index - name.len(), parser.index);
						}
					},
					FunctionDeclarationType::ClassLevel => {
						if style.module_only() {
							return FunctionDeclarationResult::Err("Style Disallowed", "style only allowed in top-level functions", parser.index - name.len(), parser.index);
						}
					},
					FunctionDeclarationType::Assumption => {
						return FunctionDeclarationResult::Err("Style Disallowed", "functions styles cannot be used with assume", parser.index - name.len(), parser.index);
					}
				}
				func_props.push(style);
			} else if name == "fn" {
				successfully_parsed = true;
				break;
			}

			// Parse Whitespace
			declare_parse_required_whitespace!(parser);
		}

		// Ensure Function Style is Parsed
		if !successfully_parsed {
			let mut temp_index = parser.index + 1;
			let chars = &parser.chars;
			while temp_index < chars.len() && chars[temp_index].is_ascii_alphabetic() { temp_index += 1; }
			return FunctionDeclarationResult::Err("Unknown Style", "unknown function style/property", parser.index, temp_index);
		}

		// Parse Whitespace
		declare_parse_required_whitespace!(parser);

		// Parse Var Name
		let mut function_name = "".to_string();
		declare_parse_required_ascii!(function_name, "Function Name Missing", "function name missing", parser);

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		let mut next_char = parser.get_curr();
		declare_parse_required_next_char!('(', next_char, parser);
		//delcare_increment!(parser);

		let mut parameters = Vec::new();
		loop {

			// Parse Whitespace
			declare_parse_whitespace!(parser);

			if parser.get_curr() == ')' {
				break;
			} else {
				let mut param_name: String;
				let mut param_type_str = "".to_string();
				let mut param_type = VarStyle::Unknown;
				declare_parse_ascii!(param_type_str, parser);

				if VarStyle::styles().contains(&param_type_str.as_str()) {
					param_type = VarStyle::new(param_type_str.as_str());

					declare_parse_required_whitespace!(parser);
					param_name = "".to_string();
					declare_parse_ascii!(param_name, parser);
				} else {
					param_type = VarStyle::Copy;
					param_name = param_type_str;
				}

				// Parse Whitespace
				declare_parse_whitespace!(parser);

				let mut next_char = parser.get_curr();
				let var_type: Type;
				if next_char == ':' {
					delcare_increment!(parser);
					declare_parse_whitespace!(parser);
					declare_parse_type!(var_type, parser);
				} else if next_char == '=' {
					var_type = Type::Inferred;
					delcare_increment!(parser);
				} else {
					return FunctionDeclarationResult::Err("Unexpected Symbol", "unexpected symbol", parser.index - 1, parser.index);
				}

				// Parse Whitespace
				declare_parse_whitespace!(parser);

				// Parse Assignment
				let mut has_value = false;
				if let Type::Inferred = var_type {
					declare_parse_required_next_char!('=', next_char, parser);
					has_value = true;
				} else if parser.get_curr() == '=' {
					has_value = true;
				}

				let mut start = parser.index;
				let mut result = ' ';
				declare_parse_expr_until_either_char!(',', ')', result, parser);
				let mut end = parser.index;
				if result != ')' && result != ',' {
					return Self::out_of_space(parser.index);
				}
				if result != ')' { delcare_increment!(parser); }

				if !has_value {
					start = 0;
					end = 0;
				}

				parameters.push((VariableType {
					var_type: var_type,
					var_style: param_type,
					var_properties: Vec::new()
				}, param_name, start, end));
			}
		}
		//parser.increment();

		delcare_increment!(parser);
		declare_parse_whitespace!(parser);

		let return_type = {
			if parser.get_curr() == '-' {
				declare_parse_required_next_char!('>', next_char, parser);
				declare_parse_whitespace!(parser);
				let var_type: Type;
				declare_parse_type!(var_type, parser);
				VariableType {
					var_type: var_type,
					var_style: VarStyle::Copy,
					var_properties: Vec::new()
				}
			} else {
				VariableType {
					var_type: Type::Void,
					var_style: VarStyle::Copy,
					var_properties: Vec::new()
				}
			}
		};

		match declare_type {
			FunctionDeclarationType::ModuleLevel | FunctionDeclarationType::ClassLevel => {

			},
			FunctionDeclarationType::Assumption => {
				let mut next_char = ' ';
				declare_parse_required_next_char!(';', next_char, parser);
			}
		}

		return FunctionDeclarationResult::Ok(FunctionDeclaration {
			name: function_name,
			props: func_props,
			parameters: parameters,
			return_type: Some(return_type),
			line: initial_line,
			start_index: 0,
			end_index: 0
		});
	}

	pub fn is_func_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		let styles = FunStyle::styles();
		for style in styles {
			if declare.starts_with(style) {
				return true;
			}
		}
		return declare.starts_with("fn ");
	}
}
