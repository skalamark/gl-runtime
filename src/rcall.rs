// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::env::Env;
use gl_core::preludes::*;

use crate::preludes::*;

impl Runtime {
	pub fn call(&self, function: Box<Expression>, arguments: Vec<Expression>) -> ResultRuntime {
		self.call_object(self.expression(*function)?, arguments)
	}

	pub fn call_object(&self, function: Object, arguments: Vec<Expression>) -> ResultRuntime {
		let mut args: Vec<Object> = Vec::new();
		for arg in arguments {
			args.push(self.expression(arg)?);
		}

		let (params, body) = match function {
			Object::Builtin(name, expect_param_num, f) =>
				return if expect_param_num < 0 || expect_param_num == args.len() as i32 {
					match f(args) {
						Ok(object) => Ok(object),
						Err(mut exception) => {
							exception.push(ExceptionPoint::new(
								self.module_context.clone(),
								Position::default(),
							));
							Err(exception)
						},
					}
				} else {
					let exception: Exception = Exception::in_runtime(Except::type_(format!(
						"{}() expected {} argument, found {}",
						&name,
						expect_param_num,
						args.len(),
					)));
					Err(exception)
				},
			Object::FnRust(name, expect_param_num, f) =>
				return if expect_param_num < 0 || expect_param_num == args.len() as i32 {
					match f(args) {
						Ok(object) => Ok(object),
						Err(mut exception) => {
							exception.push(ExceptionPoint::new(
								self.module_context.clone(),
								Position::default(),
							));
							Err(exception)
						},
					}
				} else {
					let exception: Exception = Exception::in_runtime(Except::type_(format!(
						"{}() expected {} argument, found {}",
						match name {
							Some(name_fn) => name_fn,
							None => format!("<anonymous>"),
						},
						expect_param_num,
						args.len(),
					)));
					Err(exception)
				},
			Object::Fn(name, params, body) =>
				if params.len() == args.len() {
					(params, body)
				} else {
					let exception: Exception = Exception::in_runtime(Except::type_(format!(
						"{}() expected {} argument, found {}",
						match name {
							Some(name_fn) => name_fn,
							None => format!("<anonymous>"),
						},
						params.len(),
						args.len(),
					)));
					return Err(exception);
				},
			o => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"'{}' object is not callable",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
		};

		let mut new_scoped: Env = Env::from_parent(Rc::clone(&self.env));
		for (name, o) in params.iter().zip(args) {
			new_scoped.set(name, o);
		}

		let runtime: Runtime =
			Runtime::from_env(Rc::new(RefCell::new(new_scoped)), self.module_context.clone());
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
		ast.statements = body.0;

		match runtime.run(ast) {
			Ok(object) => Ok(object),
			Err(mut exception) => {
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				Err(exception)
			},
		}
	}
}
