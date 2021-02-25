// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;

impl Runtime {
	pub fn expression(&self, expression: Expression) -> ResultRuntime {
		let left: Object = match expression {
			Expression::Identifier(identifier) => self.identifier(identifier)?,
			Expression::Literal(literal) => self.literal(literal)?,
			Expression::Prefix(prefix, right_expression) => {
				self.prefix(prefix, self.expression(*right_expression)?)?
			}
			Expression::Infix(infix, left_expression, right_expression) => self.infix(
				infix,
				self.expression(*left_expression)?,
				self.expression(*right_expression)?,
			)?,
			Expression::Call {
				function,
				arguments,
			} => self.call(function, arguments)?,
			Expression::Fn { params, body } => Object::Fn(None, params, body),
		};

		Ok(left)
	}
}
