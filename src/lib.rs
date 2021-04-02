// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

use crate::preludes::*;

mod rcall;
mod rexpression;
mod ridentifier;
mod rindex;
mod rinfix;
mod rliteral;
mod rprefix;
mod rstatement;

pub mod preludes {
	pub type ResultRuntime = Result<super::Object, super::Exception>;
	pub use crate::Runtime;
}

pub struct Runtime {
	env: Rc<RefCell<Env>>,
	module_context: String,
}

impl Runtime {
	pub fn new<T: Into<String>>(module_context: T) -> Self {
		Self { env: Rc::new(RefCell::new(Env::new())), module_context: module_context.into() }
	}

	pub fn from_env<T: Into<String>>(env: Rc<RefCell<Env>>, module_context: T) -> Self {
		Self { env, module_context: module_context.into() }
	}

	pub fn run_with_parser(&self, mut parser: Parser) -> ResultRuntime {
		let mut result: Object = Object::Null;

		loop {
			result = self.statement(match parser.next()? {
				Some(statement) => statement,
				None => break,
			})?;
		}

		Ok(result)
	}

	pub fn run(&self, ast: AbstractSyntaxTree) -> ResultRuntime {
		let mut result: Object = Object::Null;

		for statement in ast.statements {
			result = self.statement(statement)?;
		}

		Ok(result)
	}
}
