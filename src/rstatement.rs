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
				self.env.borrow_mut().set(name, value_object);
			},
			Statement::Fn { name, params, body } => {
				self.env
					.borrow_mut()
					.set(&name, Object::Fn(GFunction::new(Some(name.clone()), params, body)));
			},
			Statement::Expression(expression) => {
				let _ = self.expression(expression)?;
			},
			Statement::ExpressionReturn(expression) => result = self.expression(expression)?,
			Statement::Import(path_string) => {
				let path = std::path::Path::new(&path_string);

				if path.is_file() && path.extension().unwrap() == "gl" {
					unimplemented!()
				} else {
					let name: String = format!("{}", path.file_stem().unwrap().to_str().unwrap());
					let dynlibrary: Library = unsafe {
						match Library::new(path) {
							Ok(dynlibrary) => dynlibrary,
							Err(err) => {
								let mut exception: Exception =
									Exception::in_runtime(Except::error(err.to_string()));
								exception.push(ExceptionPoint::new(
									&self.module_context,
									Position::default(),
								));
								return Err(exception);
							},
						}
					};

					let moduledynlibrary: ModuleDynLibrary = ModuleDynLibrary::new(
						&name,
						&path_string,
						Rc::new(RefCell::new(dynlibrary)),
						Rc::new(RefCell::new(Env::new())),
					);

					if let Ok(Object::FnNative(GFunctionNative { name: _, params_len: _, body })) =
						moduledynlibrary.get_attr("init")
					{
						body(Vec::new())?;
					};

					self.env.borrow_mut().set(name, Object::ModuleDynLibrary(moduledynlibrary));
				}
			},
		}

		Ok(result)
	}
}
