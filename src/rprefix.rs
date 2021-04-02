// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

use crate::preludes::*;

impl Runtime {
	pub fn prefix(&self, prefix: Prefix, right: Object) -> ResultRuntime {
		match prefix {
			Prefix::Not => self.prefix_not_op(right),
			Prefix::Plus => self.prefix_plus_op(right),
			Prefix::Minus => self.prefix_minus_op(right),
		}
	}

	pub fn prefix_not_op(&self, right: Object) -> ResultRuntime {
		Ok(match right {
			Object::Null => Object::Boolean(true),
			Object::Boolean(true) => Object::Boolean(false),
			Object::Boolean(false) => Object::Boolean(true),
			_ => Object::Boolean(false),
		})
	}

	pub fn prefix_plus_op(&self, right: Object) -> ResultRuntime {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(integer),
			Object::Float(float) => Object::Float(float),
			Object::Boolean(boolean) => match boolean {
				true => Object::Integer(1.to_bigint().unwrap()),
				false => Object::Integer(0.to_bigint().unwrap()),
			},
			o => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"bad operand type for unary +: '{}'",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
		})
	}

	pub fn prefix_minus_op(&self, right: Object) -> ResultRuntime {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(-integer),
			Object::Float(float) => Object::Float(-float),
			Object::Boolean(boolean) => match boolean {
				true => Object::Integer(-1.to_bigint().unwrap()),
				false => Object::Integer(0.to_bigint().unwrap()),
			},
			o => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"bad operand type for unary -: '{}'",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
		})
	}
}
