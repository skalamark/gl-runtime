// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn identifier(&self, identifier: String) -> ResultRuntime {
		let env = &mut *self.env.lock().unwrap();
		match env.get_clone_object(&identifier) {
			Some(a) => return Ok(a),
			None => {
				let mut exception: Exception = Exception::new(
					Except::name(format!("name '{}' is not defined", &identifier)),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		}
	}
}
