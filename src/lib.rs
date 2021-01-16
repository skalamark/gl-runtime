// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use gl_core::error::{AnyError, Error, Exception};
use gl_core::object::Object;
use gl_core::position::Position;
use gl_core::state::ProgramState;

pub struct Runtime {}

impl Runtime {
	pub fn new() -> Self {
		Self {}
	}

	pub fn run(
		&self, ast: AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<Object, AnyError> {
		let mut result: Object = Object::Null;

		for statement in ast.statements {
			match self.statement(statement, module, program) {
				Ok(object) => result = object,
				Err(exception) => return Err(exception),
			}
		}

		Ok(result)
	}
}
