// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;

impl Runtime {
	pub fn statement(&self, statement: Statement) -> ResultRuntime {
		let mut result: Object = Object::Null;

		match statement {
			Statement::Let(name, value) => {
				let value_object: Object = match self.expression(value) {
					Ok(object) => object,
					Err(exception) => {
						self.env.borrow_mut().set(&name, Object::Null);
						return Err(exception);
					}
				};
				self.env.borrow_mut().set(&name, value_object);
			}
			Statement::Fn { name, params, body } => {
				self.env
					.borrow_mut()
					.set(&name.clone(), Object::Fn(Some(name), params, body));
			}
			Statement::Expression(expression) => {
				let _ = self.expression(expression)?;
			}
			Statement::ExpressionReturn(expression) => result = self.expression(expression)?,
		}

		Ok(result)
	}
}
