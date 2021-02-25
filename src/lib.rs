// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::env::Env;
use gl_core::preludes::*;

mod rcall;
mod rexpression;
mod ridentifier;
mod rinfix;
mod rliteral;
mod rprefix;
mod rstatement;

pub mod preludes {
	use gl_core::preludes::*;

	pub type ResultRuntime = Result<Object, Exception>;
	pub use crate::Runtime;
}

pub struct Runtime {
	env: Rc<RefCell<Env>>,
	module_context: String,
}

impl Runtime {
	pub fn new(module_context: String) -> Self {
		Self {
			env: Rc::new(RefCell::new(Env::new())),
			module_context,
		}
	}

	pub fn from_env(env: Rc<RefCell<Env>>, module_context: String) -> Self {
		Self {
			env,
			module_context,
		}
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
