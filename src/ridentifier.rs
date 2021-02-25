// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use gl_core::preludes::*;

impl Runtime {
	pub fn identifier(&self, identifier: String) -> ResultRuntime {
		match self.env.borrow().get(&identifier) {
			Some(object) => return Ok(object),
			None => {
				let mut exception: Exception = Exception::new(
					Except::name(format!("name '{}' is not defined", &identifier)),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module_context.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		}
	}
}
