// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::ops::Div;

use gl_core::preludes::*;

use crate::preludes::*;

impl Runtime {
	fn unsupported(&self, infix: Infix, left: Object, right: Object) -> ResultRuntime {
		let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
			"unsupported operand type(s) for {}: '{}' and '{}'",
			infix,
			left.typer(),
			right.typer()
		)));
		exception.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
		Err(exception)
	}

	pub fn infix(&self, infix: Infix, left: Object, right: Object) -> ResultRuntime {
		match left.clone() {
			Object::Integer(left_integer) =>
				if let Object::Integer(right_integer) = right {
					self.infix_integer(infix, left_integer, right_integer)
				} else if let Object::Float(right_float) = right {
					self.infix_integer_float(infix, left_integer, right_float)
				} else if let Object::Boolean(right_boolean) = right {
					self.infix_integer_boolean(infix, left_integer, right_boolean)
				} else {
					return self.unsupported(infix, left, right);
				},
			Object::Float(left_float) =>
				if let Object::Integer(right_integer) = right {
					self.infix_integer_float(infix, right_integer, left_float)
				} else if let Object::Float(right_float) = right {
					self.infix_float(infix, left_float, right_float)
				} else if let Object::Boolean(right_boolean) = right {
					self.infix_float_boolean(infix, left_float, right_boolean)
				} else {
					return self.unsupported(infix, left, right);
				},
			Object::Boolean(left_boolean) =>
				if let Object::Integer(right_integer) = right {
					self.infix_integer_boolean(infix, right_integer, left_boolean)
				} else if let Object::Float(right_float) = right {
					self.infix_float_boolean(infix, right_float, left_boolean)
				} else if let Object::Boolean(right_boolean) = right {
					self.infix_boolean(infix, left_boolean, right_boolean)
				} else {
					return self.unsupported(infix, left, right);
				},
			Object::String(left_string) =>
				if let Object::Integer(right_integer) = right {
					self.infix_string_integer(infix, left_string, right_integer)
				} else if let Object::Boolean(right_boolean) = right {
					self.infix_string_boolean(infix, left_string, right_boolean)
				} else if let Object::String(right_string) = right {
					self.infix_string(infix, left_string, right_string)
				} else {
					return self.unsupported(infix, left, right);
				},
			_ => {
				return self.unsupported(infix, left, right);
			},
		}
	}

	pub fn infix_integer(&self, infix: Infix, left: BigInt, right: BigInt) -> ResultRuntime {
		Ok(match infix {
			Infix::Plus => Object::Integer(left + right),
			Infix::Minus => Object::Integer(left - right),
			Infix::Multiply => Object::Integer(left * right),
			Infix::Divide => Object::Float(BigRational::from_integer(left).div(right)),
			Infix::LessThan => Object::Boolean(left < right),
			Infix::LessThanEqual => Object::Boolean(left <= right),
			Infix::GreaterThan => Object::Boolean(left > right),
			Infix::GreaterThanEqual => Object::Boolean(left >= right),
			Infix::Equal => Object::Boolean(left == right),
			Infix::NotEqual => Object::Boolean(left != right),
		})
	}

	pub fn infix_float(
		&self, infix: Infix, left: BigRational, right: BigRational,
	) -> ResultRuntime {
		Ok(match infix {
			Infix::Plus => Object::Float(left + right),
			Infix::Minus => Object::Float(left - right),
			Infix::Multiply => Object::Float(left * right),
			Infix::Divide => Object::Float(left / right),
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
				return self.unsupported(infix, Object::String(left), Object::String(right));
			},
		}
	}

	pub fn infix_boolean(&self, infix: Infix, left: bool, right: bool) -> ResultRuntime {
		let left2bigint: BigInt = match left {
			true => 1.to_bigint().unwrap(),
			false => 0.to_bigint().unwrap(),
		};
		let right2bigint: BigInt = match right {
			true => 1.to_bigint().unwrap(),
			false => 0.to_bigint().unwrap(),
		};

		self.infix_integer(infix, left2bigint, right2bigint)
	}

	pub fn infix_integer_float(
		&self, infix: Infix, left: BigInt, right: BigRational,
	) -> ResultRuntime {
		let left2float: BigRational = str_to_big_rational(&left.to_string()).unwrap();
		self.infix_float(infix, left2float, right)
	}

	pub fn infix_integer_boolean(&self, infix: Infix, left: BigInt, right: bool) -> ResultRuntime {
		let right2bigint: BigInt = match right {
			true => 1.to_bigint().unwrap(),
			false => 0.to_bigint().unwrap(),
		};
		self.infix_integer(infix, left, right2bigint)
	}

	pub fn infix_string_integer(&self, infix: Infix, left: String, right: BigInt) -> ResultRuntime {
		match infix {
			Infix::Multiply => {
				let mut result_string: String = String::new();
				let mut i: BigInt = right;
				let zero: BigInt = 0.to_bigint().unwrap();
				while i > zero {
					i = i - 1;
					result_string = format!("{}{}", result_string, &left);
				}
				Ok(Object::String(result_string))
			},
			_ => {
				return self.unsupported(infix, Object::String(left), Object::Integer(right));
			},
		}
	}

	pub fn infix_string_boolean(&self, infix: Infix, left: String, right: bool) -> ResultRuntime {
		let right2bigint: BigInt = match right {
			true => 1.to_bigint().unwrap(),
			false => 0.to_bigint().unwrap(),
		};
		self.infix_string_integer(infix, left, right2bigint)
	}

	pub fn infix_float_boolean(
		&self, infix: Infix, left: BigRational, right: bool,
	) -> ResultRuntime {
		let right2bigrational: BigRational = match right {
			true => str_to_big_rational("1").unwrap(),
			false => str_to_big_rational("0").unwrap(),
		};
		self.infix_float(infix, left, right2bigrational)
	}
}
