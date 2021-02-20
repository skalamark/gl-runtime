// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn call(&self, function: Box<Expression>, arguments: Vec<Expression>) -> ResultRuntime {
		let mut args: Vec<Arc<Mutex<Box<dyn Object>>>> = Vec::new();

		for arg in arguments {
			match self.expression(arg) {
				Ok(object) => args.push(object),
				Err(exception) => return Err(exception),
			}
		}

		let exp = match self.expression(*function) {
			Ok(a) => a,
			Err(e) => return Err(e),
		};

		return if let Some(func) = exp.lock().unwrap().downcast_mut::<FunctionRust>() {
			func.call_value(args, self.module.clone(), Position::default())
		} else {
			let mut exception: Exception =
				Exception::new_in_runtime(Except::type_(format!("object is not callable")));
			exception.push(ExceptionPoint::new(
				self.module.clone(),
				Position::default(),
			));
			return Err(exception);
		};
	}
}
