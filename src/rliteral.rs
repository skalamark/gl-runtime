// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;

impl Runtime {
	pub fn literal(&self, literal: Literal) -> ResultRuntime {
		let result: Object = match literal {
			Literal::Null => Object::Null,
			Literal::Integer(integer) => Object::Integer(integer),
			Literal::Float(float) => Object::Float(float),
			Literal::Boolean(boolean) => Object::Boolean(boolean),
			Literal::String(string) => Object::String(string),
			Literal::Vec(vector) => self.literal_vec(vector)?,
			Literal::HashMap(hashmap) => self.literal_hashmap(hashmap)?,
		};

		Ok(result)
	}

	pub fn literal_vec(&self, vector: Vec<Expression>) -> ResultRuntime {
		let mut list: Vec<Object> = Vec::new();

		for expression in vector {
			list.push(self.expression(expression)?);
		}

		Ok(Object::Vec(list))
	}

	pub fn literal_hashmap(&self, hashmap_literal: Vec<(Expression, Expression)>) -> ResultRuntime {
		let mut hashmap: HashMap<Object, Object> = HashMap::new();

		for (key_expression, value_expression) in hashmap_literal {
			let key: Object = self.expression(key_expression)?;
			let value: Object = self.expression(value_expression)?;

			hashmap.insert(key, value);
		}

		Ok(Object::HashMap(hashmap))
	}
}
