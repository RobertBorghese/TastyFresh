/**********************************************************
 * --- Context  Manager---
 *
 * Manages the individual contexts for each Tasty Fresh
 * file in the project.
 **********************************************************/

use crate::context_management::context::Context;
use crate::context_management::typing_context::ContextType;

use crate::expression::variable_type::VariableType;
use crate::expression::value_type::{ Function, ClassType };

use std::collections::BTreeMap;

pub struct ContextManager {
	pub contexts: BTreeMap<String,Context>,
	pub data_refs: BTreeMap<usize,ContextType>,
	pub max_id: usize
}

impl ContextManager {
	pub fn new() -> ContextManager {
		return ContextManager {
			contexts: BTreeMap::new(),
			data_refs: BTreeMap::new(),
			max_id: 0
		}
	}

	pub fn add_context(&mut self, id: String, ctx: Context) {
		self.contexts.insert(id, ctx);
	}

	pub fn module_exists(&self, file: &String) -> bool {
		return self.contexts.contains_key(file);
	}

	pub fn get_context_immut(&self, file: &str) -> &Context {
		return self.contexts.get(file).unwrap();
	}

	pub fn get_context(&mut self, file: &str) -> &mut Context {
		return self.contexts.get_mut(file).unwrap();
	}

	pub fn take_context(&mut self, file: &str) -> Context {
		return self.contexts.remove(file).unwrap();
	}

	pub fn get_context_type(&self, id: usize) -> Option<ContextType> {
		if self.data_refs.contains_key(&id) {
			return Some(self.data_refs.get(&id).unwrap().clone());
		}
		return None;
	}

	pub fn add_context_type(&mut self, ctx_type: ContextType) -> usize {
		self.max_id += 1;
		self.data_refs.insert(self.max_id, ctx_type);
		return self.max_id;
	}

	pub fn update_var(&mut self, id: usize, var_type: VariableType) -> bool {
		if self.data_refs.contains_key(&id) {
			self.data_refs.insert(id, ContextType::Variable(var_type));
			return true;
		}
		return false;
	}

	pub fn update_func(&mut self, id: usize, func_type: Function) -> bool {
		if self.data_refs.contains_key(&id) {
			self.data_refs.insert(id, ContextType::Function(func_type));
			return true;
		}
		return false;
	}

	pub fn update_quantum_func(&mut self, id: usize, quant_type: Vec<Function>) -> bool {
		if self.data_refs.contains_key(&id) {
			self.data_refs.insert(id, ContextType::QuantumFunction(quant_type));
			return true;
		}
		return false;
	}

	pub fn update_class(&mut self, id: usize, cls_type: ClassType) -> bool {
		if self.data_refs.contains_key(&id) {
			self.data_refs.insert(id, ContextType::Class(cls_type));
			return true;
		}
		return false;
	}
}
