/**********************************************************
 * --- Typing Context ---
 *
 * Keeps track of the types available in the current
 * parsing context.
 **********************************************************/

use std::collections::BTreeMap;

use crate::expression::variable_type::{ VariableType, Type };

use crate::expression::value_type::{ Function, ClassType };

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
	known_data: Vec<BTreeMap<String,ContextType>>
}

impl TypingContext {
	pub fn new(module_only: bool) -> TypingContext {
		let mut data = Vec::new();
		if !module_only { data.push(Self::global_data()); }
		let mut result = TypingContext {
			known_data: data
		};
		result.push_context();
		return result;
	}

	pub fn add(&mut self, ctx: &TypingContext) {
		for data in &ctx.known_data {
			self.known_data.push(data.clone());
		}
	}

	pub fn get_item(&self, name: &str) -> Option<ContextType> {
		for data in self.known_data.iter().rev() {
			if data.contains_key(name) {
				if let Some(v) = data.get(name) {
					return Some(v.clone());
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

	pub fn add_variable(&mut self, name: String, var_type: VariableType) {
		self.known_data.last_mut().unwrap().insert(name, ContextType::Variable(var_type));
	}

	pub fn add_function(&mut self, name: String, func: Function) {
		let data = self.known_data.last_mut().unwrap();
		if data.contains_key(&name) {
			let mut old_function: Option<Function> = None;
			if let ContextType::Function(old_func) = data.get(&name).unwrap() {
				old_function = Some(old_func.clone());
			}
			if let ContextType::QuantumFunction(funcs) = data.get_mut(&name).unwrap() {
				funcs.push(func);
			} else if old_function.is_some() {
				data.insert(name, ContextType::QuantumFunction(vec!(old_function.unwrap(), func)));
			}
		} else {
			data.insert(name, ContextType::Function(func));
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
