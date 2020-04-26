/**********************************************************
 * --- Typing Context ---
 *
 * Keeps track of the types available in the current
 * parsing context.
 **********************************************************/

use std::collections::BTreeMap;

use crate::expression::variable_type::{ VariableType, Type };
use crate::expression::value_type::{ Function, ClassType };

use crate::context_management::context::Context;
use crate::context_management::context_manager::ContextManager;

#[derive(Clone)]
pub enum ContextType {
	Variable(VariableType),
	Function(Function),
	QuantumFunction(Vec<Function>),
	Class(ClassType),
	PrimitiveExtension(Vec<(Type, Function)>),
	Namespace(BTreeMap<String,ContextType>)
}

#[derive(Clone)]
pub struct TypingContext {
	known_data: Vec<BTreeMap<String,usize>>,
	context_type: TypingContextType
}

#[derive(Clone)]
pub enum TypingContextType {
	ModuleLevel,
	ContentLevel(usize, BTreeMap<usize,ContextType>)
}

impl TypingContext {
	pub fn new(module_only: bool) -> TypingContext {
		let data = Vec::new();
		//if !module_only { data.push(Self::global_data()); }
		let mut result = TypingContext {
			known_data: data,
			context_type: if module_only {
				TypingContextType::ModuleLevel
			} else {
				TypingContextType::ContentLevel(0, BTreeMap::new())
			}
		};
		result.push_context();
		return result;
	}

	pub fn get_item(&self, name: &str, curr_ctx: Option<&Context>, manager: Option<&ContextManager>, recursive: bool) -> Option<ContextType> {
		for data in self.known_data.iter().rev() {
			if data.contains_key(name) {
				if let Some(v) = data.get(name) {
					if let TypingContextType::ContentLevel(_, typing_data) = &self.context_type {
						let ctx_type = typing_data.get(v);
						if ctx_type.is_some() {
							return Some(ctx_type.unwrap().clone());
						}
					} else {
						let ctx_type = manager.unwrap().get_context_type(*v);
						if ctx_type.is_some() {
							return Some(ctx_type.unwrap());
						}
					}
				}
			}
		}
		if !recursive && curr_ctx.is_some() && manager.is_some() {
			let manager_unwrap = manager.unwrap();
			for module in &curr_ctx.unwrap().shared_modules {
				let item = manager_unwrap.get_context_immut(module).module.get_item(name, curr_ctx, manager, true);
				if item.is_some() {
					return item;
				}
			}
		}
		return None;
	}

	pub fn global_data() -> BTreeMap<String,ContextType> {
		let mut result = BTreeMap::new();
		result.insert("std".to_string(), ContextType::Namespace(BTreeMap::new()));
		return result;
	}

	pub fn push_context(&mut self) {
		self.known_data.push(BTreeMap::new());
	}

	pub fn pop_context(&mut self) {
		self.known_data.pop();
	}

	pub fn register_type(&mut self, ctx_type: ContextType, manager: Option<&mut ContextManager>) -> usize {
		if let TypingContextType::ContentLevel(curr_id, typing_data) = &mut self.context_type {
			*curr_id += 1;
			typing_data.insert(*curr_id, ctx_type);
			return *curr_id;
		} else {
			return manager.unwrap().add_context_type(ctx_type);
		}
	}

	pub fn get_context_type(&mut self, data_id: usize, manager: &Option<&mut ContextManager>) -> Option<ContextType> {
		if let TypingContextType::ContentLevel(_, typing_data) = &self.context_type {
			if typing_data.contains_key(&data_id) {
				return Some(typing_data.get(&data_id).unwrap().clone());
			}
			return None;
		} else {
			return manager.as_ref().unwrap().get_context_type(data_id);
		}
	}

	pub fn add_variable(&mut self, name: String, var_type: VariableType, manager: Option<&mut ContextManager>) -> usize {
		let id = self.register_type(ContextType::Variable(var_type), manager);
		self.known_data.last_mut().unwrap().insert(name, id);
		return id;
	}

	pub fn add_function(&mut self, name: String, func: Function, manager: Option<&mut ContextManager>) -> usize {
		let mut result = 0;
		let data_contains_key = self.known_data.last_mut().unwrap().contains_key(&name);
		if data_contains_key {
			let mut old_function: Option<Function> = None;
			let data_id = self.known_data.last_mut().unwrap().get(&name).unwrap().clone();
			let ctx_type = self.get_context_type(data_id, &manager).unwrap();
			if let ContextType::Function(old_func) = &ctx_type {
				old_function = Some(old_func.clone());
			}
			if let ContextType::QuantumFunction(mut funcs) = ctx_type {
				funcs.push(func);
				//manager.update_quantum_func(funcs);
				self.update_quantum_func(data_id, funcs, manager);
				result = data_id;
			} else if old_function.is_some() {
				let funcs = vec!(old_function.unwrap(), func);
				let id = self.register_type(ContextType::QuantumFunction(funcs), manager);
				let data = self.known_data.last_mut().unwrap();
				data.insert(name, id);
				result = id;
			}
		} else {
			let id = self.register_type(ContextType::Function(func), manager);
			let data = self.known_data.last_mut().unwrap();
			//let id = manager.add_context_type(ContextType::Function(func));
			data.insert(name, id);
			result = id;
		}
		return result;
	}

	pub fn add_class(&mut self, name: String, cls_type: ClassType, manager: Option<&mut ContextManager>) -> usize {
		//let id = manager.add_context_type(ContextType::Class(cls_type));
		let id = self.register_type(ContextType::Class(cls_type), manager);
		self.known_data.last_mut().unwrap().insert(name, id);
		return id;
	}

	pub fn update_var(&mut self, id: usize, var_type: VariableType, manager: Option<&mut ContextManager>) -> bool {
		if let TypingContextType::ContentLevel(_, typing_data) = &mut self.context_type {
			if typing_data.contains_key(&id) {
				typing_data.insert(id, ContextType::Variable(var_type));
				return true;
			}
			return false;
		} else {
			return manager.unwrap().update_var(id, var_type);
		}
	}

	pub fn update_func(&mut self, id: usize, func_type: Function, manager: Option<&mut ContextManager>) -> bool {
		if let TypingContextType::ContentLevel(_, typing_data) = &mut self.context_type {
			if typing_data.contains_key(&id) {
				typing_data.insert(id, ContextType::Function(func_type));
				return true;
			}
			return false;
		} else {
			return manager.unwrap().update_func(id, func_type);
		}
	}

	pub fn update_quantum_func(&mut self, id: usize, quant_type: Vec<Function>, manager: Option<&mut ContextManager>) -> bool {
		if let TypingContextType::ContentLevel(_, typing_data) = &mut self.context_type {
			if typing_data.contains_key(&id) {
				typing_data.insert(id, ContextType::QuantumFunction(quant_type));
				return true;
			}
			return false;
		} else {
			return manager.unwrap().update_quantum_func(id, quant_type);
		}
	}

	pub fn update_class(&mut self, id: usize, cls_type: ClassType, manager: Option<&mut ContextManager>) -> bool {
		if let TypingContextType::ContentLevel(_, typing_data) = &mut self.context_type {
			if typing_data.contains_key(&id) {
				typing_data.insert(id, ContextType::Class(cls_type));
				return true;
			}
			return false;
		} else {
			return manager.unwrap().update_class(id, cls_type);
		}
	}

	pub fn print_everything(&self) {
		for data in &self.known_data {
			println!("-- SCOPE --");
			for (key, _value) in data.iter() {
				println!("{}", key);
			}
		}
	}
}
