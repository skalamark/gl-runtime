// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

use crate::preludes::*;

impl Runtime {
	pub fn expression(&self, expression: Expression) -> ResultRuntime {
		match expression {
			Expression::Identifier(identifier) => self.identifier(identifier),
			Expression::Literal(literal) => self.literal(literal),
			Expression::Prefix(prefix, right_expression) =>
				self.prefix(prefix, self.expression(*right_expression)?),
			Expression::Infix(infix, left_expression, right_expression) => self.infix(
				infix,
				self.expression(*left_expression)?,
				self.expression(*right_expression)?,
			),
			Expression::Fn { params, body } => Ok(Object::Fn(None, params, body)),
			Expression::Call { function, arguments } => self.call(function, arguments),
			Expression::Index(left_expression, index_expression) =>
				self.index(self.expression(*left_expression)?, self.expression(*index_expression)?),
		}
	}
}
