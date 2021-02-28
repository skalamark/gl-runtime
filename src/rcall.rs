// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::env::Env;
use gl_core::preludes::*;

impl Runtime {
	pub fn call(&self, function: Box<Expression>, arguments: Vec<Expression>) -> ResultRuntime {
		let mut args: Vec<Object> = Vec::new();

		for arg in arguments {
			args.push(self.expression(arg)?);
		}

		let (name, params, body) = match self.expression(*function) {
			Ok(Object::Builtin(name, expect_param_num, f)) => {
				if expect_param_num < 0 || expect_param_num == args.len() as i32 {
					return f(args, self.module_context.clone(), Position::new(0, 0));
				} else {
					let mut exception: Exception = Exception::new(
						Except::type_(format!(
							"{}() takes {} positional argument but {} were given",
							&name,
							expect_param_num,
							args.len(),
						)),
						true,
					);
					exception.push(ExceptionPoint::new(
						self.module_context.clone(),
						Position::default(),
					));
					return Err(exception);
				}
			}
			Ok(Object::Fn(name, params, body)) => (name, params, body),
			Ok(_) => {
				let mut exception: Exception =
					Exception::new(Except::type_(format!("object is not callable")), true);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
			Err(exception) => return Err(exception),
		};

		if params.len() != args.len() {
			let mut exception: Exception = Exception::new(
				Except::type_(format!(
					"{}{} takes {} positional argument but {} were given",
					if let Some(name_fn) = name {
						name_fn
					} else {
						format!("<anonymous>")
					},
					{
						let mut params_string: String = String::new();
						params_string.push_str(&format!("("));
						for (i, param) in params.iter().enumerate() {
							params_string.push_str(&format!("{}", param));
							if i < params.len() - 1 {
								params_string.push_str(", ");
							}
						}
						params_string.push_str(&format!(")"));
						params_string
					},
					params.len(),
					args.len(),
				)),
				true,
			);
			exception.push(ExceptionPoint::new(
				self.module_context.clone(),
				Position::default(),
			));
			return Err(exception);
		}

		let mut scoped_env: Env = Env::new_with_parent(Rc::clone(&self.env));
		for (name, o) in params.iter().zip(args) {
			scoped_env.set(name, o);
		}

		let runtime: Runtime = Runtime::from_env(
			Rc::new(RefCell::new(scoped_env)),
			self.module_context.clone(),
		);
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
		ast.statements = body.0;

		let object: Object = match runtime.run(ast) {
			Ok(object) => object,
			Err(mut exception) => {
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		};

		Ok(object)
	}
}
