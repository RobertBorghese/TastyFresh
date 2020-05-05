/**********************************************************
 * --- Attributes ---
 *
 * A data structure for storing and analysing the 
 * attributes of content within Tasty Fresh.
 **********************************************************/

use crate::context_management::global_context::GlobalContext;

use crate::declaration_parser::attribute_declaration::AttributeDeclaration;

#[derive(Clone)]
pub struct Attributes {
	data: Option<Vec<AttributeDeclaration>>
}

impl Attributes {
	pub fn new(data: Option<Vec<AttributeDeclaration>>) -> Attributes {
		return Attributes {
			data: data
		};
	}

	pub fn has_attribute(&self, name: &str) -> bool {
		if self.data.is_some() {
			for a in self.data.as_ref().unwrap() {
				if a.name == name {
					return true;
				}
			}
		}
		return false;
	}

	pub fn get_attribute(&self, name: &str) -> Option<&AttributeDeclaration> {
		if self.data.is_some() {
			for a in self.data.as_ref().unwrap() {
				if a.name == name {
					return Some(a);
				}
			}
		}
		return None;
	}

	pub fn get_attribute_param_length(&self, name: &str) -> usize {
		let attr = self.get_attribute(name);
		if attr.is_some() {
			return attr.unwrap().params_length();
		}
		return 0;
	}

	pub fn get_attribute_parameters(&self, name: &str, content: &str) -> Vec<String> {
		let mut result = Vec::new();
		if self.data.is_some() {
			for a in self.data.as_ref().unwrap() {
				if a.name == name {
					for i in 0..a.params_length() {
						result.push(a.get_param(i, content));
					}
					break;
				}
			}
		}
		return result;
	}

	pub fn flatten_attributes(&mut self, global_context: &GlobalContext, content: &str) {
		if self.data.is_some() {
			let attributes = self.data.as_mut().unwrap();
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

	pub fn get_required_includes(&self) -> Vec<(String,bool)> {
		let mut result = Vec::new();
		if self.has_attribute("RequireInclude") {
			let params = self.get_attribute("RequireInclude");
			if params.is_some() {
				let param_params = &params.as_ref().unwrap().parameters;
				if param_params.is_some() {
					let p = param_params.as_ref().unwrap();
					let first = p.first();
					if first.is_some() {
						let first_unwrap = first.as_ref().unwrap();
						let right = first_unwrap.as_ref().right();
						if right.is_some() {
							let inc = right.as_ref().unwrap().to_string();
							let sys = p.len() <= 1;
							result.push((inc, sys));
						}
					}
				}
			}
		}
		return result;
	}
}
