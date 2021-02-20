// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Runtime {
	pub fn expression(&self, expression: Expression) -> ResultRuntime {
		let left: Arc<Mutex<Box<dyn Object>>> = match expression {
			Expression::Identifier(identifier) => self.identifier(identifier)?,
			Expression::Literal(literal) => self.literal(literal)?,
			Expression::Prefix(prefix, right_expression) => {
				self.prefix(prefix, self.expression(*right_expression)?)?
			}
			Expression::Infix(infix, left_expression, right_expression) => self.infix(
				infix,
				self.expression(*left_expression)?,
				self.expression(*right_expression)?,
			)?,
			// Expression::Index(left_expression, index_expression) => {
			// 	let left = match self.expression(*left_expression, module) {
			// 		Ok(object) => object,
			// 		Err(exception) => return Err(exception),
			// 	};
			// 	let index = match self.expression(*index_expression, module) {
			// 		Ok(object) => object,
			// 		Err(exception) => return Err(exception),
			// 	};
			// 	match self.index(left, index, module) {
			// 		Ok(object) => object,
			// 		Err(exception) => return Err(exception),
			// 	}
			// }
			Expression::Call {
				function,
				arguments,
			} => self.call(function, arguments)?,
			// Expression::Fn { params, body } => Object::Fn(None, params, body),
			// Expression::If {
			// 	condition,
			// 	consequence,
			// 	alternative,
			// } => match self.if_(*condition, consequence, alternative, module) {
			// 	Ok(object) => object,
			// 	Err(exception) => return Err(exception),
			// },
			Expression::Attribute(object, o2) => match *object {
				Expression::Identifier(name) => match self.identifier(name) {
					Ok(objects) => match *o2 {
						Expression::Identifier(name2) => {
							let m = &mut objects.lock().unwrap();

							match m.getattribute(format!("{}", &name2)) {
								Some(f) => Arc::new(Mutex::new(f)),
								None => {
									let mut exception: Exception = Exception::new(
										Except::name(format!(
											"'' object has no attribute '{}'",
											&name2
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
						Expression::Call {
							function,
							arguments,
						} => match *function {
							Expression::Identifier(name2) => {
								let m = &mut objects.lock().unwrap();

								let mut args: Vec<Arc<Mutex<Box<dyn Object>>>> = Vec::new();

								for arg in arguments {
									match self.expression(arg) {
										Ok(object) => args.push(object),
										Err(exception) => return Err(exception),
									}
								}

								return m.call(
									name2.clone(),
									args,
									self.module.clone(),
									Position::default(),
								);
							}
							_ => unimplemented!(),
						},
						a => {
							println!("{:#?}k", a);
							unimplemented!()
						}
					},
					Err(e) => return Err(e),
				},
				_ => unimplemented!(),
			},
			_ => unimplemented!(),
		};

		Ok(left)
	}
}
