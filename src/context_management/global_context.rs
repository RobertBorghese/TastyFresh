/**********************************************************
 * --- Global Context ---
 *
 * Keeps track of globally available components like
 * attributes and abstracts.
 **********************************************************/

use crate::declaration_parser::attribute_class_declaration::AttributeClassDeclaration;

pub struct GlobalContext {
	pub attribute_classes: Vec<AttributeClassDeclaration>
}

impl GlobalContext {
	pub fn new() -> GlobalContext {
		return GlobalContext {
			attribute_classes: Vec::new()
		};
	}

	pub fn add_attribute_class(&mut self, cls: AttributeClassDeclaration) {
		self.attribute_classes.push(cls);
	}

	pub fn find_attribute(&self, name: &str) -> Option<&AttributeClassDeclaration> {
		for a in &self.attribute_classes {
			if a.name == name {
				return Some(a);
			}
		}
		return None;
	}
}
