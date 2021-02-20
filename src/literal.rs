// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn literal(&self, literal: Literal) -> ResultRuntime {
		let result: Box<dyn Object> = match literal {
			Literal::Null => Box::new(Null::new()),
			Literal::Integer(integer) => Box::new(Integer::new(integer)),
			Literal::Float(float) => Box::new(Float::new(float)),
			Literal::Boolean(boolean) => Box::new(Boolean::new(boolean)),
			Literal::String(string) => Box::new(StringLiteral::new(string)),
			Literal::Vec(vector) => match self.literal_vec(vector) {
				Ok(object) => return Ok(object),
				Err(exception) => return Err(exception),
			},
			Literal::HashMap(hashmap) => match self.literal_hashmap(hashmap) {
				Ok(object) => return Ok(object),
				Err(exception) => return Err(exception),
			},
		};

		Ok(Arc::new(Mutex::new(result)))
	}

	pub fn literal_vec(&self, expressions: Vec<Expression>) -> ResultRuntime {
		let mut list = Vec::new();

		for expression in expressions {
			match self.expression(expression) {
				Ok(object) => list.push(object),
				Err(exception) => return Err(exception),
			}
		}

		Ok(Arc::new(Mutex::new(Box::new(Vector::new_with_value(list)))))
	}

	pub fn literal_hashmap(&self, hashmap_literal: Vec<(Expression, Expression)>) -> ResultRuntime {
		let mut hashmap: HashMap<String, Arc<Mutex<Box<dyn Object>>>> = HashMap::new();

		for (key_expression, value_expression) in hashmap_literal {
			let key = self.expression(key_expression)?;
			let value = self.expression(value_expression)?;
			let mut k = format!("");

			if let Some(string) = key.lock().unwrap().downcast_ref::<StringLiteral>() {
				k = string.value.clone()
			}

			hashmap.insert(k, value);
		}

		Ok(Arc::new(Mutex::new(Box::new(HashMapGL::new_with_value(
			hashmap,
		)))))
	}
}
