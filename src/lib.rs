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

	fn literal(
		&self, literal: Literal, module: &String, program: &mut ProgramState,
	) -> Result<Object, AnyError> {
		let result: Object = match literal {
			Literal::Null => Object::Null,
			Literal::Integer(integer) => Object::Integer(integer),
			Literal::Boolean(boolean) => Object::Boolean(boolean),
			Literal::String(string) => Object::String(string),
		};

		Ok(result)
	}

	fn identifier(
		&self, identifier: String, module: &String, program: &mut ProgramState,
	) -> Result<Object, AnyError> {
		match program.env.get(&identifier, module) {
			Some(object) => return Ok(object.clone()),
			None => {
				return Err(Exception::new(
					module.clone(),
					Position::default(),
					Error::name(format!("name '{}' is not defined", &identifier)),
				));
			}
		}
	}

	fn expression(
		&self, expression: Expression, module: &String, program: &mut ProgramState,
	) -> Result<Object, AnyError> {
		let left: Object = match expression {
			Expression::Identifier(identifier) => {
				match self.identifier(identifier, module, program) {
					Ok(object) => object,
					Err(exception) => return Err(exception),
				}
			}
			Expression::Literal(literal) => match self.literal(literal, module, program) {
				Ok(object) => object,
				Err(exception) => return Err(exception),
			},
		};

		Ok(left)
	}

	fn statement(
		&self, statement: Statement, module: &String, program: &mut ProgramState,
	) -> Result<Object, AnyError> {
		let mut result: Object = Object::Null;

		match statement {
			Statement::Let(name, value) => {
				let value_object: Object = match self.expression(value, module, program) {
					Ok(object) => object,
					Err(exception) => return Err(exception),
				};
				program.env.set(&name, value_object, module);
			}
			Statement::Expression(expression) => {
				match self.expression(expression, module, program) {
					Ok(object) => result = object,
					Err(exception) => return Err(exception),
				}
			}
		}

		Ok(result)
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
