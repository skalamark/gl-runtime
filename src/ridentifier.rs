// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use gl_core::preludes::*;

use crate::preludes::*;

impl Runtime {
	pub fn identifier<T: Into<String>>(&self, identifier: T) -> ResultRuntime {
		let identifier: String = identifier.into();

		match self.env.borrow().get(identifier.clone()) {
			Some(object) => return Ok(object),
			None => {
				let mut exception: Exception = Exception::in_runtime(Except::name(format!(
					"name '{}' is not defined",
					identifier
				)));
				exception
					.push(ExceptionPoint::new(self.module_context.clone(), Position::default()));
				return Err(exception);
			},
		}
	}
}
