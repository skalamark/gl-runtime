// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;
use libloading::Library;

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
			Statement::Import(name) => {
				let dynlibrary: Library = unsafe { Library::new(name.clone()).unwrap() };
				let mut moduledynlibrary: ModuleDynLibrary =
					ModuleDynLibrary::new(name.clone(), dynlibrary, HashMap::new());
				let init: fn(HashMap<String, Object>) -> Result<(), Exception> =
					moduledynlibrary.get_function(format!("init"))?;
				init(HashMap::new())?;

				let namemoduledynlibrary: String = moduledynlibrary.get_name();
				let value_object: Object = Object::ModuleDynLibrary(moduledynlibrary);
				self.env
					.borrow_mut()
					.set(&namemoduledynlibrary, value_object);
			}
		}

		Ok(result)
	}
}
