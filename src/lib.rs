// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use gl_core::error::{Exception, ExceptionError, ExceptionMain};
use gl_core::object::Object;
use gl_core::position::Position;
use gl_core::state::ProgramState;
use std::collections::HashMap;

type ResultRuntime = Result<Object, ExceptionMain>;

pub struct Runtime {}

impl Runtime {
	pub fn new() -> Self {
		Self {}
	}

	fn literal_hashmap(
		&self, hashmap_literal: Vec<(Expression, Expression)>, module: &String,
		program: &mut ProgramState,
	) -> ResultRuntime {
		let mut hashmap: HashMap<Object, Object> = HashMap::new();

		for (key_expression, value_expression) in hashmap_literal {
			let key: Object = match self.expression(key_expression, module, program) {
				Ok(object) => object,
				Err(exception) => return Err(exception),
			};

			let value: Object = match self.expression(value_expression, module, program) {
				Ok(object) => object,
				Err(exception) => return Err(exception),
			};

			hashmap.insert(key, value);
		}

		Ok(Object::HashMap(hashmap))
	}

	fn literal_vec(
		&self, vector: Vec<Expression>, module: &String, program: &mut ProgramState,
	) -> ResultRuntime {
		let mut list: Vec<Object> = Vec::new();

		for expression in vector {
			match self.expression(expression, module, program) {
				Ok(object) => list.push(object),
				Err(exception) => return Err(exception),
			}
		}

		Ok(Object::Vec(list))
	}

	fn literal(
		&self, literal: Literal, module: &String, program: &mut ProgramState,
	) -> ResultRuntime {
		let result: Object = match literal {
			Literal::Null => Object::Null,
			Literal::Integer(integer) => Object::Integer(integer),
			Literal::Boolean(boolean) => Object::Boolean(boolean),
			Literal::String(string) => Object::String(string),
			Literal::Vec(vector) => match self.literal_vec(vector, module, program) {
				Ok(object) => object,
				Err(exception) => return Err(exception),
			},
			Literal::HashMap(hashmap) => match self.literal_hashmap(hashmap, module, program) {
				Ok(object) => object,
				Err(exception) => return Err(exception),
			},
		};

		Ok(result)
	}

	fn identifier(
		&self, identifier: String, module: &String, program: &mut ProgramState,
	) -> ResultRuntime {
		match program.env.get(&identifier, module) {
			Some(object) => return Ok(object.clone()),
			None => {
				let mut exception = ExceptionMain::new(
					ExceptionError::name(format!("name '{}' is not defined", &identifier)),
					true,
				);
				exception.push(Exception::new(module.clone(), Position::default()));
				return Err(exception);
			}
		}
	}

	fn expression(
		&self, expression: Expression, module: &String, program: &mut ProgramState,
	) -> ResultRuntime {
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
	) -> ResultRuntime {
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
	) -> ResultRuntime {
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
