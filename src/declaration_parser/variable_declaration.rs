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
use crate::expression::value_type::{ Function, Property };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::context_management::typing_context::Context;

use crate::config_management::operator_data::OperatorDataStructure;

type VariableDeclarationResult = DeclarationResult<VariableDeclaration>;

pub struct VariableDeclaration {
	pub name: String,
	pub var_type: VariableType,
	pub line: usize,
	pub value: Option<(usize, usize)>
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

		// Parse Variable Properties and Style
		let mut name = "".to_string();
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
			declare_parse_whitespace!(parser);
			declare_parse_type!(var_type, parser);
			declare_parse_whitespace!(parser);
			next_char = parser.get_curr();
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
			value: value
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_var_declaration(&parser.content, parser.index);
	}

	pub fn is_var_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		let styles = VarStyle::styles();
		for style in styles {
			if declare.starts_with(style) {
				return true;
			}
		}
		let props = VarProps::properties();
		for prop in props {
			if declare.starts_with(prop) {
				return true;
			}
		}
		return false;
	}

	pub fn to_cpp(&self, expr: &Option<Rc<Expression>>, operators: &OperatorDataStructure, context: &mut Context) -> String {
		let var_type = &self.var_type;
		let default_value = var_type.default_value();
		return if expr.is_some() {
			format!("{} {} = {};", var_type.to_cpp(), self.name, expr.as_ref().unwrap().to_string(operators, context))
		} else if default_value.is_some() {
			format!("{} {} = {};", var_type.to_cpp(), self.name, default_value.unwrap())
		} else {
			format!("{} {};", var_type.to_cpp(), self.name)
		};
	}
}
