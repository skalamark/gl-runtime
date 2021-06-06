// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

use crate::preludes::*;

impl Runtime {
	pub fn infix(&self, infix: Infix, left: Object, right: Object) -> ResultRuntime {
		match {
			match infix {
				Infix::Plus => left + right,
				Infix::Minus => left - right,
				Infix::Multiply => left * right,
				Infix::Divide => left / right,
				Infix::Equal => Ok(Object::Boolean(left == right)),
				Infix::NotEqual => Ok(Object::Boolean(left != right)),
				Infix::LessThan => Ok(Object::Boolean(left < right)),
				Infix::LessThanEqual => Ok(Object::Boolean(left <= right)),
				Infix::GreaterThan => Ok(Object::Boolean(left > right)),
				Infix::GreaterThanEqual => Ok(Object::Boolean(left >= right)),
			}
		} {
			Ok(r) => Ok(r),
			Err(mut exception) => {
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				Err(exception)
			},
		}
	}
}
