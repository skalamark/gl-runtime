// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;
use libloading::Library;

use crate::preludes::*;

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
					},
				};
				self.env.borrow_mut().set(&name, value_object);
			},
			Statement::Fn { name, params, body } => {
				self.env.borrow_mut().set(&name.clone(), Object::Fn(Some(name), params, body));
			},
			Statement::Expression(expression) => {
				let _ = self.expression(expression)?;
			},
			Statement::ExpressionReturn(expression) => result = self.expression(expression)?,
			Statement::Import(path) => {
				let dynlibrary: Library = unsafe {
					match Library::new(path.clone()) {
						Ok(dynlibrary) => dynlibrary,
						Err(err) => {
							let mut exception: Exception =
								Exception::in_runtime(Except::error(err.to_string()));
							exception.push(ExceptionPoint::new(
								self.module_context.clone(),
								Position::default(),
							));
							return Err(exception);
						},
					}
				};
				let name: String = format!(
					"{}",
					std::path::Path::new(&path).file_stem().unwrap().to_str().unwrap()
				);
				let mut moduledynlibrary: ModuleDynLibrary =
					ModuleDynLibrary::new(name.clone(), path, dynlibrary, HashMap::new());

				if let Ok(init) = moduledynlibrary.get_function("init") {
					init(Vec::new())?;
				};

				self.env.borrow_mut().set(name, Object::ModuleDynLibrary(moduledynlibrary));
			},
		}

		Ok(result)
	}
}
