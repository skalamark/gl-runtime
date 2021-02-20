// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

mod block;
mod call;
mod expression;
mod identifier;
mod infix;
mod literal;
mod prefix;
mod statement;

mod preludes {
	pub type ResultRuntime = Result<Arc<Mutex<Box<dyn Object>>>, Exception>;
	pub use crate::Runtime;
	pub use gl_core::preludes::*;
}

pub struct Runtime {
	env: Arc<Mutex<Env>>,
	module: String,
}

impl Runtime {
	pub fn new(module: &String) -> Self {
		Self {
			env: Arc::new(Mutex::new(Env::new())),
			module: module.clone(),
		}
	}

	pub fn new_from_env(env: Arc<Mutex<Env>>, module: &String) -> Self {
		Self {
			env,
			module: module.clone(),
		}
	}

	pub fn run_with_parser(
		&self, mut parser: Parser, _: &mut ProgramState,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, gl_core::error::Exception> {
		let mut result: Arc<Mutex<Box<dyn Object>>> = Arc::new(Mutex::new(Box::new(Null::new())));

		loop {
			result = self.statement(match parser.next()? {
				Some(statement) => statement,
				None => break,
			})?;
		}

		Ok(result)
	}

	pub fn run(
		&self, ast: AbstractSyntaxTree, _: &mut ProgramState,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, gl_core::error::Exception> {
		let mut result: Arc<Mutex<Box<dyn Object>>> = Arc::new(Mutex::new(Box::new(Null::new())));

		for statement in ast.statements {
			result = self.statement(statement)?;
		}

		Ok(result)
	}
}
