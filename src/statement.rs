// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn statement(&self, statement: Statement) -> ResultRuntime {
		let mut result: Arc<Mutex<Box<dyn Object>>> = Arc::new(Mutex::new(Box::new(Null::new())));

		match statement {
			Statement::Let(name, value) => {
				let value_object = match self.expression(value) {
					Ok(object) => object,
					Err(exception) => {
						let env = &mut *self.env.lock().unwrap();
						env.set(&name, Arc::new(Mutex::new(Box::new(Null::new()))));
						return Err(exception);
					}
				};
				{
					let env = &mut *self.env.lock().unwrap();
					env.set(&name, value_object);
				}
			}
			// Statement::Fn { name, params, body } => {
			// 	self.env
			// 		.borrow_mut()
			// 		.set(&name.clone(), Object::Fn(Some(name), params, body));
			// }
			Statement::Expression(expression) => {
				let _ = self.expression(expression)?;
			}
			Statement::ExpressionReturn(expression) => result = self.expression(expression)?,
			_ => unimplemented!(),
		}

		Ok(result)
	}
}
