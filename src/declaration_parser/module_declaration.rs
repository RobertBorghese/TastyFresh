/**********************************************************
 * --- Module Declaration ---
 *
 * Represents the declarations contained within an
 * individual Tasty Fresh source file.
 **********************************************************/

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::variable_declaration::VariableDeclaration;
use crate::declaration_parser::function_declaration::FunctionDeclaration;

pub enum DeclarationType {
	Attribute(AttributeDeclaration),
	Variable(VariableDeclaration),
	Function(FunctionDeclaration)
}

pub struct ModuleDeclaration {
	pub declarations: Vec<DeclarationType>
}

impl ModuleDeclaration {
	pub fn new(parser: &Parser) {

	}
}