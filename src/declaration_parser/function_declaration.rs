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
	declare_parse_required_ascii_op,
	declare_parse_required_next_char,
	declare_parse_expr_until_next_char,
	declare_parse_expr_until_either_char,
	declare_parse_type,
	declare_parse_type_and_style,
	delcare_increment
};

use crate::config_management::operator_data::OperatorDataStructure;

use crate::expression::variable_type::{ VariableType, Type, VarStyle };
use crate::expression::value_type::{ Function, Property };
use crate::expression::function_type::FunStyle;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

use regex::Regex;

lazy_static! {
	pub static ref FUNC_STYLE_REGEX: Regex = Regex::new(r"^\b(?:static|extern|virtual|inline|meta)\b").unwrap();
	pub static ref FUNC_REGEX: Regex = Regex::new(r"^\b(?:fn|op|constructor|destructor)\b").unwrap();
}

type FunctionDeclarationResult = DeclarationResult<FunctionDeclaration>;

#[derive(Clone)]
pub struct FunctionDeclaration {
	pub name: String,
	pub props: Vec<FunStyle>,
	pub parameters: Vec<(VariableType, String, Option<usize>, Option<usize>, bool)>,
	pub return_type: VariableType,
	pub function_type: FunctionType,
	pub line: usize,
	pub start_index: Option<usize>,
	pub end_index: Option<usize>,
	pub declaration_id: usize
}

#[derive(Clone)]
pub enum FunctionType {
	Normal,
	Operator(String, usize),
	Constructor,
	Destructor
}

impl FunctionType {
	pub fn is_normal_or_operator(&self) -> bool {
		return self.is_normal() || self.is_operator();
	}

	pub fn is_constructor_or_destructor(&self) -> bool {
		return self.is_constructor() || self.is_destructor();
	}

	pub fn is_normal(&self) -> bool {
		if let FunctionType::Normal = self {
			return true;
		}
		return false;
	}

	pub fn is_operator(&self) -> bool {
		if let FunctionType::Operator(..) = self {
			return true;
		}
		return false;
	}

	pub fn get_operator_type(&self) -> String {
		if let FunctionType::Operator(op_type, _) = self {
			return op_type.clone();
		}
		return "".to_string();
	}

	pub fn get_operator_id(&self) -> usize {
		if let FunctionType::Operator(_, id) = self {
			return *id;
		}
		return 0;
	}

	pub fn is_constructor(&self) -> bool {
		if let FunctionType::Constructor = self {
			return true;
		}
		return false;
	}

	pub fn is_destructor(&self) -> bool {
		if let FunctionType::Destructor = self {
			return true;
		}
		return false;
	}
}

pub enum FunctionDeclarationType {
	ModuleLevel,
	ClassLevel,
	Assumption
}

impl FunctionDeclarationType {
	pub fn is_class(&self) -> bool {
		if let FunctionDeclarationType::ClassLevel = self {
			return true;
		}
		return false;
	}

	pub fn is_assumption(&self) -> bool {
		if let FunctionDeclarationType::Assumption = self {
			return true;
		}
		return false;
	}
}

impl Declaration<FunctionDeclaration> for FunctionDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl CPPTranspiler for FunctionDeclaration {
	fn to_cpp(&self) -> String {
		return "".to_string();
	}
}

impl FunctionDeclaration {
	pub fn new(parser: &mut Parser, declare_type: FunctionDeclarationType, operator_data: Option<&OperatorDataStructure>) -> FunctionDeclarationResult {
		let initial_line = parser.line;

		let mut func_type = FunctionType::Normal;
		let mut func_props = Vec::new();

		// Parse Function Properties and Style
		let mut successfully_parsed = false;
		let mut name;
		let mut is_extern = false;
		while Self::is_func_declaration(&parser.content, parser.index) {
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
				if let FunStyle::Extern = style {
					is_extern = true;
				}
				func_props.push(style);
			} else if name == "fn" {
				successfully_parsed = true;
				break;
			} else if name == "op" {
				if !declare_type.is_class() {
					return FunctionDeclarationResult::Err("Operators Disallowed", "operator functions can only be used in classes", parser.index - name.len(), parser.index);
				}
				func_type = FunctionType::Operator("".to_string(), 100);
				successfully_parsed = true;
				break;
			} else if name == "constructor" {
				if !declare_type.is_class() {
					return FunctionDeclarationResult::Err("Constructors Disallowed", "constructors can only be used in classes", parser.index - name.len(), parser.index);
				}
				func_type = FunctionType::Constructor;
				successfully_parsed = true;
				break;
			} else if name == "destructor" {
				if !declare_type.is_class() {
					return FunctionDeclarationResult::Err("Destructor Disallowed", "destructors can only be used in classes", parser.index - name.len(), parser.index);
				}
				func_type = FunctionType::Destructor;
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
		if func_type.is_normal_or_operator() {
			declare_parse_required_whitespace!(parser);
		} else {
			declare_parse_whitespace!(parser);
		}

		// Parse Var Name
		let mut function_name = "".to_string();
		if func_type.is_normal_or_operator() {
			if func_type.is_operator() {
				declare_parse_required_ascii_op!(function_name, "Invalid Operator", "operator requires valid operator symbols", parser);
			} else {
				declare_parse_required_ascii!(function_name, "Function Name Missing", "function name missing", parser);
			}
		}

		if func_type.is_operator() {
			if operator_data.is_none() {
				panic!("No operator data available!");
			}
			let mut found_operator = false;
			for (op_type, ops) in operator_data.unwrap() {
				let mut index = 0;
				for op in ops {
					if *op.name.as_ref().unwrap() == function_name {
						func_type = FunctionType::Operator(op_type.to_string(), index);
						found_operator = true;
						break;
					}
					index += 1;
				}
			}
			if !found_operator {
				return FunctionDeclarationResult::Err("Unknown Operator", "unknown operator", parser.index, parser.index);
			}
		}

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		let mut next_char = parser.get_curr();
		let mut parameters = Vec::new();
		if !func_type.is_destructor() {

			declare_parse_required_next_char!('(', next_char, parser);

			loop {
				// Parse Whitespace
				declare_parse_whitespace!(parser);

				if parser.get_curr() == ')' {
					break;
				} else {
					let mut is_declare = false;
					let mut param_name: String;
					let mut param_type_str = "".to_string();
					let param_type;
					declare_parse_ascii!(param_type_str, parser);

					if param_type_str == "declare" {
						is_declare = true;
						declare_parse_required_whitespace!(parser);
						param_type_str = "".to_string();
						declare_parse_ascii!(param_type_str, parser);
					}

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

					let mut has_value = false;
					let next_char = parser.get_curr();
					let var_type: Type;
					if next_char == ':' {
						delcare_increment!(parser);
						declare_parse_whitespace!(parser);
						declare_parse_type!(var_type, parser);
					} else if next_char == '=' {
						var_type = Type::Inferred;
						delcare_increment!(parser);
						has_value = true;
					} else {
						return FunctionDeclarationResult::Err("Unexpected Symbol", "unexpected symbol", parser.index - 1, parser.index);
					}

					// Parse Whitespace
					declare_parse_whitespace!(parser);

					// Parse Assignment
					if parser.get_curr() == '=' {
						delcare_increment!(parser);
						has_value = true;
						declare_parse_whitespace!(parser);
					}

					let mut start = None;
					let mut end = None;
					if has_value {
						start = Some(parser.index);
						let mut result = ' ';
						declare_parse_expr_until_either_char!(',', ')', result, parser);
						end = Some(parser.index);
						if result != ')' && result != ',' {
							return Self::out_of_space(parser.index);
						}
					}

					declare_parse_whitespace!(parser);

					if parser.get_curr() != ')' { delcare_increment!(parser); }

					parameters.push((VariableType {
						var_type: var_type,
						var_style: param_type,
						var_properties: None,
						var_optional: false
					}, param_name, start, end, is_declare));
				}
			}
			//parser.increment();

			delcare_increment!(parser);
			declare_parse_whitespace!(parser);

		} // !is_destructor

		let return_type = {
			if parser.get_curr() == '-' && func_type.is_normal_or_operator() {
				delcare_increment!(parser);
				declare_parse_required_next_char!('>', next_char, parser);
				declare_parse_whitespace!(parser);
				let var_type: Type;
				let var_style: VarStyle;
				declare_parse_type_and_style!(var_type, var_style, parser);
				VariableType {
					var_type: var_type,
					var_style: var_style,
					var_properties: None,
					var_optional: false
				}
			} else {
				VariableType {
					var_type: Type::Void,
					var_style: VarStyle::Copy,
					var_properties: None,
					var_optional: false
				}
			}
		};

		declare_parse_whitespace!(parser);

		let mut start_index: Option<usize> = None;
		let mut end_index: Option<usize> = None;

		if is_extern || declare_type.is_assumption() {
			let mut next_char = ' ';
			declare_parse_required_next_char!(';', next_char, parser);
		} else {
			let mut next_char = ' ';
			declare_parse_required_next_char!('{', next_char, parser);
			start_index = Some(parser.index);
			declare_parse_expr_until_next_char!('}', parser);
			end_index = Some(parser.index);
		}

		return FunctionDeclarationResult::Ok(FunctionDeclaration {
			name: function_name,
			props: func_props,
			parameters: parameters,
			return_type: return_type,
			function_type: func_type,
			line: initial_line,
			start_index: start_index,
			end_index: end_index,
			declaration_id: 0
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_func_declaration(&parser.content, parser.index);
	}

	pub fn is_func_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		if FUNC_STYLE_REGEX.is_match(declare) {
			return true;
		}
		return FUNC_REGEX.is_match(declare);
	}

	pub fn to_function(&self, content: &str) -> Function {
		let mut params = Vec::new();
		for param in self.parameters.clone() {
			params.push(Property {
				name: param.1,
				prop_type: param.0,
				default_value: if param.2.is_some() && param.3.is_some() {
					Some(content[param.2.unwrap()..param.3.unwrap()].to_string())
				} else {
					None
				},
				is_declare: param.4
			});
		}

		return Function {
			name: self.name.clone(),
			parameters: params,
			return_type: self.return_type.clone(),
			styles: self.props.clone()
		}
	}

	pub fn header_only(&self) -> bool {
		for s in &self.props {
			if s.is_extern() {
				return true;
			}
		}
		return false;
	}
}
