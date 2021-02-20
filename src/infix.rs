// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn infix(
		&self, infix: Infix, left: Arc<Mutex<Box<dyn Object>>>, right: Arc<Mutex<Box<dyn Object>>>,
	) -> ResultRuntime {
		let typer_left: String = left.lock().unwrap().typer().to_string();
		let typer_right: String = right.lock().unwrap().typer().to_string();

		if let Some(left_integer) = left.lock().unwrap().downcast_ref::<Integer>() {
			if let Some(right_integer) = right.lock().unwrap().downcast_ref::<Integer>() {
				Ok(match infix {
					Infix::Plus => Arc::new(Mutex::new(Box::new(Integer::new(
						left_integer.value.clone() + right_integer.value.clone(),
					)))),
					Infix::Minus => Arc::new(Mutex::new(Box::new(Integer::new(
						left_integer.value.clone() - right_integer.value.clone(),
					)))),
					Infix::Multiply => Arc::new(Mutex::new(Box::new(Integer::new(
						left_integer.value.clone() * right_integer.value.clone(),
					)))),
					Infix::Divide => Arc::new(Mutex::new(Box::new(Integer::new(
						left_integer.value.clone() / right_integer.value.clone(),
					)))),
					Infix::LessThan => Arc::new(Mutex::new(Box::new(Boolean::new(
						left_integer.value.clone() < right_integer.value.clone(),
					)))),
					Infix::LessThanEqual => Arc::new(Mutex::new(Box::new(Boolean::new(
						left_integer.value.clone() <= right_integer.value.clone(),
					)))),
					Infix::GreaterThan => Arc::new(Mutex::new(Box::new(Boolean::new(
						left_integer.value.clone() > right_integer.value.clone(),
					)))),
					Infix::GreaterThanEqual => Arc::new(Mutex::new(Box::new(Boolean::new(
						left_integer.value.clone() >= right_integer.value.clone(),
					)))),
					Infix::Equal => Arc::new(Mutex::new(Box::new(Boolean::new(
						left_integer.value.clone() == right_integer.value.clone(),
					)))),
					Infix::NotEqual => Arc::new(Mutex::new(Box::new(Boolean::new(
						left_integer.value.clone() != right_integer.value.clone(),
					)))),
				})
			} else {
				let mut exception = Exception::new(
					Except::type_(format!(
						"'{}' not supported between instances of '{}' and '{}'",
						infix, typer_left, typer_right,
					)),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		} else if let Some(left_string) = left.lock().unwrap().downcast_ref::<StringLiteral>() {
			if let Some(right_string) = right.lock().unwrap().downcast_ref::<StringLiteral>() {
				Ok(match infix {
					Infix::Plus => Arc::new(Mutex::new(Box::new(StringLiteral::new(format!(
						"{}{}",
						left_string, right_string
					))))),
					_ => {
						let mut exception = Exception::new(
							Except::type_(format!(
								"'{}' not supported between instances of '{}' and '{}'",
								infix, typer_left, typer_right,
							)),
							true,
						);
						exception.push(ExceptionPoint::new(
							self.module.clone(),
							Position::default(),
						));
						return Err(exception);
					}
				})
			} else if let Some(right_integer) = right.lock().unwrap().downcast_ref::<Integer>() {
				Ok(match infix {
					Infix::Multiply => {
						let mut result_string: String = String::new();
						let mut i = right_integer.value.clone();
						let zero = BigInt::parse_bytes(b"0", 10).unwrap();
						while i > zero {
							i = i - 1;
							result_string = format!("{}{}", result_string, left_string.clone());
						}
						Arc::new(Mutex::new(Box::new(StringLiteral::new(result_string))))
					}
					_ => {
						let mut exception = Exception::new(
							Except::type_(format!(
								"'{}' not supported between instances of '{}' and '{}'",
								infix, typer_left, typer_right,
							)),
							true,
						);
						exception.push(ExceptionPoint::new(
							self.module.clone(),
							Position::default(),
						));
						return Err(exception);
					}
				})
			} else {
				let mut exception = Exception::new(
					Except::type_(format!(
						"'{}' not supported between instances of '{}' and '{}'",
						infix, typer_left, typer_right,
					)),
					true,
				);
				exception.push(ExceptionPoint::new(
					self.module.clone(),
					Position::default(),
				));
				return Err(exception);
			}
		} else {
			let mut exception = Exception::new(
				Except::type_(format!(
					"'{}' not supported between instances of '{}' and '{}'",
					infix, typer_left, typer_right,
				)),
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
