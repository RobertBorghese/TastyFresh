/**********************************************************
 * --- Static Extension ---
 *
 * Tracks the static extensions (using refurbish or abstract)
 * and applies them when necessary.
 **********************************************************/

use crate::expression::variable_type::{ VariableType, Type };
use crate::expression::value_type::Function;

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct StaticExtensionContext {
	pub extensions: BTreeMap<String,Vec<StaticExtension>>
}

impl StaticExtensionContext {
	pub fn new() -> StaticExtensionContext {
		return StaticExtensionContext {
			extensions: BTreeMap::new()
		}
	}

	pub fn insert(&mut self, func_name: String, extend: StaticExtension) {
		if self.extensions.contains_key(&func_name) {
			self.extensions.get_mut(&func_name).unwrap().push(extend);
		} else {
			self.extensions.insert(func_name, vec![extend]);
		}
	}

	pub fn find(&self, func_name: &str, t: &VariableType) -> Option<StaticExtension> {
		if self.extensions.contains_key(func_name) {
			let possibilities = self.extensions.get(func_name);
			if possibilities.is_some() {
				for e in possibilities.unwrap() {
					if e.is_type(t) {
						return Some(e.clone());
					}
				}
			}
		}
		return None;
	}
}

#[derive(Clone)]
pub struct StaticExtension {
	pub name: String,
	pub func: Function,
	pub extend_type: VariableType
}

impl StaticExtension {
	pub fn new(name: String, func: Function, extend_type: VariableType) -> StaticExtension {
		return StaticExtension {
			name: name,
			func: func,
			extend_type: extend_type
		};
	}

	pub fn is_type(&self, t: &VariableType) -> bool {
		if self.extend_type.is_equal(t) {
			return true;
		}
		if (self.extend_type.var_style.is_inferred()) && self.extend_type.var_type == t.var_type {
			return true;
		}
		if let Type::Undeclared(names) = &self.extend_type.var_type {
			if names.len() == 1 {
				if let Type::Class(cls_type) = &t.var_type {
					return cls_type.name == *names.first().unwrap();
				}
			}
		}
		return false;
	}
}
