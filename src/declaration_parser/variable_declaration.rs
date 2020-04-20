/**********************************************************
 * --- Variable Declaration ---
 *
 * Represents a variable declaration prior to being parsed.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_ascii,
	declare_parse_required_next_char,
	declare_parse_expr_until_next_char,
	declare_parse_type,
	delcare_increment
};

use std::rc::Rc;

use crate::expression::Expression;
use crate::expression::variable_type::{ VariableType, Type, VarStyle, VarProps };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::context_management::context::Context;

use crate::config_management::operator_data::OperatorDataStructure;

type VariableDeclarationResult = DeclarationResult<VariableDeclaration>;

pub struct VariableDeclaration {
	pub name: String,
	pub var_type: VariableType,
	pub line: usize,
	pub value: Option<(usize, usize)>,
	pub pure_assign: bool
}

impl Declaration<VariableDeclaration> for VariableDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of variable";
	}
}

impl VariableDeclaration {
	pub fn new(parser: &mut Parser) -> VariableDeclarationResult {
		let initial_line = parser.line;

		let mut var_props = Vec::new();
		let mut var_style = VarStyle::Unknown;
		let mut pure_assign = false;

		// Parse Variable Properties and Style
		let mut name;
		while Self::is_var_declaration(&parser.content, parser.index) {
			name = "".to_string();
			declare_parse_ascii!(name, parser);
			if VarProps::properties().contains(&name.as_str()) {
				var_props.push(VarProps::new(name.as_str()));
			} else if VarStyle::styles().contains(&name.as_str()) {
				var_style = VarStyle::new(name.as_str());
				break;
			}

			// Parse Whitespace
			declare_parse_required_whitespace!(parser);
		}

		// Ensure Variable Style is Parsed
		if var_style.is_unknown() {
			let mut temp_index = parser.index + 1;
			let chars = &parser.chars;
			while temp_index < chars.len() && chars[temp_index].is_ascii_alphabetic() { temp_index += 1; }
			return VariableDeclarationResult::Err("Unknown Style", "unknown variable style/property", parser.index, temp_index);
		}

		// Parse Whitespace
		declare_parse_required_whitespace!(parser);

		// Parse Var Name
		let mut variable_name = "".to_string();
		declare_parse_required_ascii!(variable_name, "Variable Name Missing", "variable name missing", parser);

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		// Parse Var Type
		let mut next_char = parser.get_curr();
		let mut has_value = true;
		let var_type: Type;
		if next_char == ':' {
			delcare_increment!(parser);
			next_char = parser.get_curr();
			if next_char != '=' {
				declare_parse_whitespace!(parser);
				declare_parse_type!(var_type, parser);
				declare_parse_whitespace!(parser);
				next_char = parser.get_curr();
			} else {
				pure_assign = true;
				var_type = Type::Inferred;
			}
			if next_char == ':' {
				pure_assign = true;
				delcare_increment!(parser);
				next_char = parser.get_curr();
			}
			if next_char == '=' {
				declare_parse_required_next_char!('=', next_char, parser);
			} else if next_char == ';' {
				has_value = false;
			} else {
				return VariableDeclarationResult::Err("Unexpected Symbol", "unexpected symbol", parser.index - 1, parser.index);
			}
		} else if next_char == '=' {
			var_type = Type::Inferred;
			delcare_increment!(parser);
		} else if next_char == ';' {
			return VariableDeclarationResult::Err("Unknown Variable Type", "variable needs known type given explicitly or through value", parser.index - variable_name.len() - 1, parser.index - 1);
		} else {
			return VariableDeclarationResult::Err("Unexpected Symbol", "unexpected symbol", parser.index - 1, parser.index);
		}

		// Parse Expression
		let mut value: Option<(usize, usize)> = None;
		if has_value {
			let start = parser.index;
			declare_parse_expr_until_next_char!(';', parser);
			let end = parser.index;
			value = Some((start, end));
		}

		return VariableDeclarationResult::Ok(VariableDeclaration {
			name: variable_name,
			var_type: VariableType {
				var_type: var_type,
				var_style: var_style,
				var_properties: Some(var_props),
				var_optional: false
			},
			line: initial_line,
			value: value,
			pure_assign: pure_assign
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_var_declaration(&parser.content, parser.index);
	}

	pub fn is_var_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		let props = VarProps::properties();
		for prop in props {
			if declare.starts_with(prop) {
				return true;
			}
		}
		let styles = VarStyle::styles();
		for style in styles {
			if declare.starts_with(style) {
				return true;
			}
		}
		return false;
	}

	pub fn is_only_static(&self) -> bool {
		return self.var_type.is_only_static();
	}

	pub fn to_cpp(&self,
		expr: &Option<Rc<Expression>>,
		operators: &OperatorDataStructure,
		context: &mut Context,
		export_type: VariableExportType,
		declare_type: bool
	) -> String {

		let var_type = &self.var_type;
		let default_value = var_type.default_value();
		let props = if var_type.var_properties.is_some() && !export_type.is_class_source() {
			let mut result = Vec::new();
			for prop in var_type.var_properties.as_ref().unwrap() {
				result.push(prop.get_name());
			}
			if result.is_empty() {
				"".to_string()
			} else {
				format!("{} ", result.join(" "))
			}
		} else {
			"".to_string()
		};

		let final_name = if !export_type.is_class_source() { self.name.to_string() } else { format!("{}::{}", export_type.get_source_name(), self.name) };

		if expr.is_some() {

			let is_construction = expr.as_ref().unwrap().is_construction_call();

			if is_construction {
				let var_type_output = var_type.to_cpp(declare_type);
				let var_type_name = var_type.var_type.to_cpp(false);
				let params = expr.as_ref().unwrap().get_parameters(operators, context);
				let params_str = params.join(", ");
				match var_type.var_style {
					VarStyle::Copy => {
						if export_type.is_class_header() {
							return format!("{}{} {} = {}({});", props, var_type_output, final_name, var_type_name, params_str);
						} else {
							if params.is_empty() {
								return format!("{}{} {};", props, var_type_output, final_name);
							} else {
								return format!("{}{} {}({});", props, var_type_output, final_name, params_str);
							}
						}
					},
					VarStyle::Move => {
						return format!("{}{}&& {} = {}({});", props, var_type_output, final_name, var_type_name, params_str);
					},
					VarStyle::Ptr(self_size) => {
						if self_size == 1 {
							return format!("{}{} {} = new {}({});", props, var_type_output, final_name, var_type_name, params_str);
						}
					},
					VarStyle::AutoPtr => {
						return format!("{}std::shared_ptr<{}> {} = std::make_shared<{}>({});", props, var_type_output, final_name, var_type_name, params_str);
					},
					VarStyle::UniquePtr => {
						return format!("{}std::unique_ptr<{}> {} = std::make_unique<{}>({});", props, var_type_output, final_name, var_type_name, params_str);
					},
					_ => ()
				}
			}

			let right_str = expr.as_ref().unwrap().to_string(operators, context);
			return format!("{}{} {} = {};",
				props,
				var_type.to_cpp(declare_type),
				final_name,
				if self.pure_assign {
					right_str
				} else {
					expr.as_ref().unwrap().get_type().convert_between_styles(var_type, &right_str).unwrap_or(right_str.to_string())
				}
			);
		} else if default_value.is_some() {
			return format!("{}{} {} = {};", props, var_type.to_cpp(declare_type), final_name, default_value.unwrap());
		} else {
			return format!("{}{} {};", props, var_type.to_cpp(declare_type), final_name);
		};
	}
}

pub enum VariableExportType<'a> {
	Scoped,
	ModuleHeader,
	ModuleSource,
	ClassHeader,
	ClassSource(&'a str)
}

impl<'a> VariableExportType<'a> {
	pub fn is_class_header(&self) -> bool {
		if let VariableExportType::ClassHeader = self {
			return true;
		}
		return false;
	}

	pub fn is_class_source(&self) -> bool {
		if let VariableExportType::ClassSource(_) = self {
			return true;
		}
		return false;
	}

	pub fn get_source_name(&self) -> &'a str {
		if let VariableExportType::ClassSource(name) = self {
			return name;
		}
		return "";
	}
}
