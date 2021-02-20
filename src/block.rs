// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn block(&self, block: Block) -> ResultRuntime {
		let mut result: Arc<Mutex<Box<dyn Object>>> = Arc::new(Mutex::new(Box::new(Null::new())));
		let Block(statements) = block;

		for statement in statements {
			match self.statement(statement) {
				Ok(object) => result = object,
				Err(exception) => return Err(exception),
			}
		}

		Ok(result)
	}
}
