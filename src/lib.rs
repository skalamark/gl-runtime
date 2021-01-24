// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::ast::{AbstractSyntaxTree, Expression, Infix, Literal, Prefix, Statement};
use gl_core::error::{Exception, ExceptionError, ExceptionMain};
use gl_core::object::Object;
use gl_core::position::Position;
use gl_core::state::ProgramState;
use gl_core::utils::Env;
use num::BigInt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type ResultRuntime = Result<Object, ExceptionMain>;

pub struct Runtime {
	env: Rc<RefCell<Env>>,
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			env: Rc::new(RefCell::new(Env::new())),
		}
	}

	pub fn new_from_env(env: Rc<RefCell<Env>>) -> Self {
		Self { env }
	}

	fn _is_truthy(object: &Object) -> bool {
		match object {
			Object::Null | Object::Boolean(false) => false,
			_ => true,
		}
	}

	fn call(
		&self, function: Box<Expression>, arguments: Vec<Expression>, module: &String,
		program: &mut ProgramState,
	) -> ResultRuntime {
		let mut args: Vec<Object> = Vec::new();

		for arg in arguments {
			match self.expression(arg, module, program) {
				Ok(object) => args.push(object),
				Err(exception) => return Err(exception),
			}
		}

		let (name, params, body) = match self.expression(*function, module, program) {
			Ok(Object::Builtin(name, expect_param_num, f)) => {
				if expect_param_num < 0 || expect_param_num == args.len() as i32 {
					return f(args, module.clone(), Position::new(0, 0));
				} else {
					let mut exception = ExceptionMain::new(
						ExceptionError::type_(format!(
							"{}() takes {} positional argument but {} were given",
							&name,
							expect_param_num,
							args.len(),
						)),
						true,
					);
					exception.push(Exception::new(module.clone(), Position::default()));
					return Err(exception);
				}
			}
			Ok(Object::Fn(name, params, body)) => (name, params, body),
			Ok(_) => {
				let mut exception = ExceptionMain::new(
					ExceptionError::type_(format!("object is not callable")),
					true,
				);
				exception.push(Exception::new(module.clone(), Position::default()));
				return Err(exception);
			}
			Err(exception) => return Err(exception),
		};

		if params.len() != args.len() {
			let mut exception = ExceptionMain::new(
				ExceptionError::type_(format!(
					"{}() takes {} positional argument but {} were given",
					if let Some(name_fn) = name {
						name_fn
					} else {
						format!("<anonymous>")
					},
					params.len(),
					args.len(),
				)),
				true,
			);
			exception.push(Exception::new(module.clone(), Position::default()));
			return Err(exception);
		}

		let mut scoped_env: Env = Env::new_with_parent(Rc::clone(&self.env));
		let list = params.iter().zip(args.iter());
		for (_, (name, o)) in list.enumerate() {
			scoped_env.set(name, o.clone());
		}
		let runtime: Runtime = Runtime::new_from_env(Rc::new(RefCell::new(scoped_env)));
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
		ast.statements = body.0;

		let object: Object = match runtime.run(ast, module, program) {
			Ok(object) => object,
			Err(mut exception) => {
				exception.push(Exception::new(module.clone(), Position::default()));
				return Err(exception);
			}
		};

		Ok(object)
	}

	fn infix(
		&self, infix: Infix, left: Object, right: Object, module: &String, _: &mut ProgramState,
	) -> ResultRuntime {
		match left {
			Object::Integer(left_integer) => {
				if let Object::Integer(right_integer) = right {
					Ok(match infix {
						Infix::Plus => Object::Integer(left_integer + right_integer),
						Infix::Minus => Object::Integer(left_integer - right_integer),
						Infix::Multiply => Object::Integer(left_integer * right_integer),
						Infix::Divide => Object::Integer(left_integer / right_integer),
						Infix::LessThan => Object::Boolean(left_integer < right_integer),
						Infix::LessThanEqual => Object::Boolean(left_integer <= right_integer),
						Infix::GreaterThan => Object::Boolean(left_integer > right_integer),
						Infix::GreaterThanEqual => Object::Boolean(left_integer >= right_integer),
						Infix::Equal => Object::Boolean(left_integer == right_integer),
						Infix::NotEqual => Object::Boolean(left_integer != right_integer),
					})
				} else {
					let mut exception = ExceptionMain::new(
						ExceptionError::type_(format!("unsupported operand type(s) for {}", infix)),
						true,
					);
					exception.push(Exception::new(module.clone(), Position::default()));
					return Err(exception);
				}
			}
			Object::String(left_string) => {
				if let Object::String(right_string) = right {
					Ok(match infix {
						Infix::Plus => Object::String(format!("{}{}", left_string, right_string)),
						_ => {
							let mut exception = ExceptionMain::new(
								ExceptionError::type_(format!(
									"unsupported operand type(s) for {}",
									infix
								)),
								true,
							);
							exception.push(Exception::new(module.clone(), Position::default()));
							return Err(exception);
						}
					})
				} else if let Object::Integer(right_integer) = right {
					Ok(match infix {
						Infix::Multiply => {
							let mut result_string: String = String::new();
							let mut i = right_integer;
							let zero = BigInt::parse_bytes(b"0", 10).unwrap();
							while i > zero {
								i = i - 1;
								result_string = format!("{}{}", result_string, left_string.clone());
							}
							Object::String(result_string)
						}
						_ => {
							let mut exception = ExceptionMain::new(
								ExceptionError::type_(format!(
									"unsupported operand type(s) for {}",
									infix
								)),
								true,
							);
							exception.push(Exception::new(module.clone(), Position::default()));
							return Err(exception);
						}
					})
				} else {
					let mut exception = ExceptionMain::new(
						ExceptionError::type_(format!("unsupported operand type(s) for {}", infix)),
						true,
					);
					exception.push(Exception::new(module.clone(), Position::default()));
					return Err(exception);
				}
			}
			_ => {
				let mut exception = ExceptionMain::new(
					ExceptionError::type_(format!("unsupported operand type(s) for +")),
					true,
				);
				exception.push(Exception::new(module.clone(), Position::default()));
				return Err(exception);
			}
		}
	}

	fn prefix_minus_op(
		&self, right: Object, module: &String, _: &mut ProgramState,
	) -> ResultRuntime {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(-integer),
			_ => {
				let mut exception = ExceptionMain::new(
					ExceptionError::type_(format!("bad operand type for unary -")),
					true,
				);
				exception.push(Exception::new(module.clone(), Position::default()));
				return Err(exception);
			}
		})
	}

	fn prefix_plus_op(
		&self, right: Object, module: &String, _: &mut ProgramState,
	) -> ResultRuntime {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(integer),
			_ => {
				let mut exception = ExceptionMain::new(
					ExceptionError::type_(format!("bad operand type for unary +")),
					true,
				);
				exception.push(Exception::new(module.clone(), Position::default()));
				return Err(exception);
			}
		})
	}

	fn prefix_not_op(&self, right: Object, _: &String, _: &mut ProgramState) -> ResultRuntime {
		Ok(match right {
			Object::Boolean(true) => Object::Boolean(false),
			Object::Boolean(false) => Object::Boolean(true),
			Object::Null => Object::Boolean(true),
			_ => Object::Boolean(false),
		})
	}

	fn prefix(
		&self, prefix: Prefix, right: Object, module: &String, program: &mut ProgramState,
	) -> ResultRuntime {
		match prefix {
			Prefix::Not => self.prefix_not_op(right, module, program),
			Prefix::Plus => self.prefix_plus_op(right, module, program),
			Prefix::Minus => self.prefix_minus_op(right, module, program),
		}
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
		&self, identifier: String, module: &String, _: &mut ProgramState,
	) -> ResultRuntime {
		match self.env.borrow().get(&identifier) {
			Some(object) => return Ok(object.clone()),
			None => {
				let mut exception: ExceptionMain = ExceptionMain::new(
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
			Expression::Prefix(prefix, right_expression) => {
				match self.expression(*right_expression, module, program) {
					Ok(right) => match self.prefix(prefix, right, module, program) {
						Ok(object) => object,
						Err(exception) => return Err(exception),
					},
					Err(exception) => return Err(exception),
				}
			}
			Expression::Infix(infix, left_expression, right_expression) => {
				match self.expression(*left_expression, module, program) {
					Ok(left) => match self.expression(*right_expression, module, program) {
						Ok(right) => match self.infix(infix, left, right, module, program) {
							Ok(object) => object,
							Err(exception) => return Err(exception),
						},
						Err(exception) => return Err(exception),
					},
					Err(exception) => return Err(exception),
				}
			}
			Expression::Call {
				function,
				arguments,
			} => match self.call(function, arguments, module, program) {
				Ok(object) => object,
				Err(exception) => return Err(exception),
			},
			Expression::Fn { params, body } => Object::Fn(None, params, body),
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
					Err(exception) => {
						self.env.borrow_mut().set(&name, Object::Null);
						return Err(exception);
					}
				};
				self.env.borrow_mut().set(&name, value_object);
			}
			Statement::Fn { name, params, body } => {
				self.env
					.borrow_mut()
					.set(&name.clone(), Object::Fn(Some(name), params, body));
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
