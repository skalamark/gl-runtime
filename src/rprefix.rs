// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;

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
			Object::Boolean(true) => Object::Boolean(false),
			Object::Boolean(false) => Object::Boolean(true),
			Object::Null => Object::Boolean(true),
			_ => Object::Boolean(false),
		})
	}

	pub fn prefix_plus_op(&self, right: Object) -> ResultRuntime {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(integer),
			_ => {
				let mut exception =
					Exception::new(Except::type_(format!("bad operand type for unary +")), true);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		})
	}

	pub fn prefix_minus_op(&self, right: Object) -> ResultRuntime {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(-integer),
			_ => {
				let mut exception =
					Exception::new(Except::type_(format!("bad operand type for unary -")), true);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		})
	}
}
