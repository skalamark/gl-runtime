// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

use crate::preludes::*;
impl Runtime {
	pub fn index(&self, left: Object, index: Object) -> ResultRuntime {
		match (left, index) {
			(Object::Vec(vector), Object::Integer(integer)) => self.index_vec(vector, integer),
			(Object::Vec(_), index) => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"list indices must be integers, not {}",
					index.typer()
				)));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
			(Object::HashMap(hashmap), index) => match hashmap.get(&index) {
				Some(object) => Ok(object.clone()),
				None => {
					let mut exception: Exception =
						Exception::in_runtime(Except::key(format!("{}", index)));
					exception.push(ExceptionPoint::new(
						self.module_context.clone(),
						Position::default(),
					));
					return Err(exception);
				},
			},
			(o, _) => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"'{}' object is not subscriptable",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
		}
	}

	pub fn index_vec(&self, vector: Vec<Object>, index: BigInt) -> ResultRuntime {
		if index < 0.to_bigint().unwrap() {
			let mut exception: Exception =
				Exception::in_runtime(Except::type_(format!("vec index out of range")));
			exception.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
			return Err(exception);
		}

		match vector.get(index.to_usize().unwrap()) {
			Some(object) => Ok(object.clone()),
			None => {
				let mut exception: Exception =
					Exception::in_runtime(Except::index(format!("vec index out of range")));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
		}
	}
}
