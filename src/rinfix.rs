// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;

impl Runtime {
	pub fn infix(&self, infix: Infix, left: Object, right: Object) -> ResultRuntime {
		match left {
			Object::Integer(left_integer) => {
				if let Object::Integer(right_integer) = right {
					self.infix_integer(infix, left_integer, right_integer)
				} else {
					let mut exception: Exception = Exception::new(
						Except::type_(format!("unsupported operand type(s) for {}", infix)),
						true,
					);
					exception.push(ExceptionPoint::new(
						self.module_context.clone(),
						Position::default(),
					));
					return Err(exception);
				}
			}
			Object::String(left_string) => {
				if let Object::String(right_string) = right {
					self.infix_string(infix, left_string, right_string)
				} else if let Object::Integer(right_integer) = right {
					self.infix_string_integer(infix, left_string, right_integer)
				} else {
					let mut exception = Exception::new(
						Except::type_(format!("unsupported operand type(s) for {}", infix)),
						true,
					);
					exception.push(ExceptionPoint::new(
						self.module_context.clone(),
						Position::default(),
					));
					return Err(exception);
				}
			}
			_ => {
				let mut exception = Exception::new(
					Except::type_(format!("unsupported operand type(s) for +")),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		}
	}

	pub fn infix_integer(&self, infix: Infix, left: BigInt, right: BigInt) -> ResultRuntime {
		Ok(match infix {
			Infix::Plus => Object::Integer(left + right),
			Infix::Minus => Object::Integer(left - right),
			Infix::Multiply => Object::Integer(left * right),
			Infix::Divide => Object::Integer(left / right),
			Infix::LessThan => Object::Boolean(left < right),
			Infix::LessThanEqual => Object::Boolean(left <= right),
			Infix::GreaterThan => Object::Boolean(left > right),
			Infix::GreaterThanEqual => Object::Boolean(left >= right),
			Infix::Equal => Object::Boolean(left == right),
			Infix::NotEqual => Object::Boolean(left != right),
		})
	}

	pub fn infix_string(&self, infix: Infix, left: String, right: String) -> ResultRuntime {
		match infix {
			Infix::Plus => Ok(Object::String(format!("{}{}", left, right))),
			_ => {
				let mut exception: Exception = Exception::new(
					Except::type_(format!("unsupported operand type(s) for {}", infix)),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		}
	}

	pub fn infix_string_integer(&self, infix: Infix, left: String, right: BigInt) -> ResultRuntime {
		match infix {
			Infix::Multiply => {
				let mut result_string: String = String::new();
				let mut i: BigInt = right;
				let zero: BigInt = 0.to_bigint().unwrap();
				while i > zero {
					i = i - 1;
					result_string = format!("{}{}", result_string, left.clone());
				}
				Ok(Object::String(result_string))
			}
			_ => {
				let mut exception: Exception = Exception::new(
					Except::type_(format!("unsupported operand type(s) for {}", infix)),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		}
	}
}
