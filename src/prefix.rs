// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn prefix_minus_op(&self, right: Arc<Mutex<Box<dyn Object>>>) -> ResultRuntime {
		if let Some(integer) = right.lock().unwrap().downcast_ref::<Integer>() {
			Ok(Arc::new(Mutex::new(Box::new(Integer::new(
				-integer.value.clone(),
			)))))
		} else {
			let mut exception =
				Exception::new(Except::type_(format!("bad operand type for unary -")), true);
			exception.push(ExceptionPoint::new(
				self.module.clone(),
				Position::default(),
			));
			return Err(exception);
		}
	}

	pub fn prefix_plus_op(&self, right: Arc<Mutex<Box<dyn Object>>>) -> ResultRuntime {
		if let Some(integer) = right.lock().unwrap().downcast_ref::<Integer>() {
			Ok(Arc::new(Mutex::new(Box::new(Integer::new(
				integer.value.clone(),
			)))))
		} else {
			let mut exception =
				Exception::new(Except::type_(format!("bad operand type for unary +")), true);
			exception.push(ExceptionPoint::new(
				self.module.clone(),
				Position::default(),
			));
			return Err(exception);
		}
	}

	pub fn prefix_not_op(&self, right: Arc<Mutex<Box<dyn Object>>>) -> ResultRuntime {
		if let Some(boolean) = right.lock().unwrap().downcast_ref::<Boolean>() {
			if boolean.value == true {
				Ok(Arc::new(Mutex::new(Box::new(Boolean::new(false)))))
			} else {
				Ok(Arc::new(Mutex::new(Box::new(Boolean::new(true)))))
			}
		} else if let Some(_) = right.lock().unwrap().downcast_ref::<Null>() {
			Ok(Arc::new(Mutex::new(Box::new(Boolean::new(true)))))
		} else {
			Ok(Arc::new(Mutex::new(Box::new(Boolean::new(false)))))
		}
	}

	pub fn prefix(&self, prefix: Prefix, right: Arc<Mutex<Box<dyn Object>>>) -> ResultRuntime {
		match prefix {
			Prefix::Not => self.prefix_not_op(right),
			Prefix::Plus => self.prefix_plus_op(right),
			Prefix::Minus => self.prefix_minus_op(right),
		}
	}
}
