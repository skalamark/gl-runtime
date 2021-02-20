// fn _is_truthy(object: &Object) -> bool {
// 	match object {
// 		Object::Null => false,
// 		Object::Boolean(boolean) if boolean.boolean == false => false,
// 		_ => true,
// 	}
// }

// fn if_(
// 	&self, condition_expression: Expression, consequence: Block,
// 	alternative_option: Option<Block>, module: &String, program: &mut ProgramState,
// ) -> ResultRuntime {
// 	let condition: Object = match self.expression(condition_expression, module, program) {
// 		Ok(object) => object,
// 		Err(exception) => return Err(exception),
// 	};

// 	if Self::_is_truthy(&condition) {
// 		match self.block(consequence, module, program) {
// 			Ok(object) => Ok(object),
// 			Err(exception) => return Err(exception),
// 		}
// 	} else if let Some(alternative) = alternative_option {
// 		match self.block(alternative, module, program) {
// 			Ok(object) => Ok(object),
// 			Err(exception) => return Err(exception),
// 		}
// 	} else {
// 		Ok(Object::Null)
// 	}
// }

// fn index_vec(
// 	&self, vector: Vec<Object>, index: BigInt, module: &String, _: &mut ProgramState,
// ) -> ResultRuntime {
// 	if index < BigInt::parse_bytes(b"0", 10).unwrap() {
// 		let mut exception = ExceptionMain::new(
// 			ExceptionError::type_(format!("`integer: Neg` is not satisfied")),
// 			true,
// 		);
// 		exception.push(Exception::new(module.clone(), Position::default()));
// 		return Err(exception);
// 	}

// 	match vector.get(index.to_string().parse::<usize>().unwrap()) {
// 		Some(object) => Ok(object.copy()),
// 		None => {
// 			let mut exception = ExceptionMain::new(
// 				ExceptionError::index(format!("vec index out of range")),
// 				true,
// 			);
// 			exception.push(Exception::new(module.clone(), Position::default()));
// 			return Err(exception);
// 		}
// 	}
// }

// fn index(
// 	&self, left: Object, index: Object, module: &String, program: &mut ProgramState,
// ) -> ResultRuntime {
// 	match left {
// 		Object::Vec(vector) => {
// 			if let Object::Integer(integer) = index {
// 				match self.index_vec(vector, integer.integer, module, program) {
// 					Ok(object) => Ok(object),
// 					Err(exception) => return Err(exception),
// 				}
// 			} else {
// 				let mut exception = ExceptionMain::new(
// 					ExceptionError::type_(format!(
// 						"list indices must be integers, not {}",
// 						index.typer()
// 					)),
// 					true,
// 				);
// 				exception.push(Exception::new(module.clone(), Position::default()));
// 				return Err(exception);
// 			}
// 		}
// 		Object::HashMap(hashmap) => match hashmap.get(&index) {
// 			Some(object) => Ok(object.copy()),
// 			None => {
// 				let mut exception =
// 					ExceptionMain::new(ExceptionError::key(format!("{}", &index)), true);
// 				exception.push(Exception::new(module.clone(), Position::default()));
// 				return Err(exception);
// 			}
// 		},
// 		_ => {
// 			let mut exception = ExceptionMain::new(
// 				ExceptionError::type_(format!(
// 					"'{}' object is not subscriptable",
// 					left.typer()
// 				)),
// 				true,
// 			);
// 			exception.push(Exception::new(module.clone(), Position::default()));
// 			return Err(exception);
// 		}
// 	}
// }

Object::BuiltinRust {
	name,
	len_args,
	builtinfn,
		}) => {
			if len_args < 0 || len_args == args.len() as i32 {
				return builtinfn(args, module.clone(), Position::new(0, 0));
			} else {
				let mut exception = Exception::new(
					Except::type_(format!(
						"{}() takes {} positional argument but {} were given",
						&name,
						len_args,
						args.len(),
					)),
					true,
				);
				exception.push(ExceptionPoint::new(module.clone(), Position::default()));
				return Err(exception);
			}
		}

		Ok(Object::Fn(name, params, body)) => (name, params, body),

		Object::FnRust {
			name,
			len_args,
			fnrust,
		}) => {
			if len_args < 0 || len_args == args.len() as i32 {
				return fnrust(args, module.clone(), Position::new(0, 0));
			} else {
				let mut exception = Exception::new(
					Except::type_(format!(
						"{}() takes {} positional argument but {} were given",
						&name,
						len_args,
						args.len(),
					)),
					true,
				);
				exception.push(ExceptionPoint::new(module.clone(), Position::default()));
				return Err(exception);
			}
		}




		let mut scoped_env = Env::new_with_parent(Rc::clone(&self.env));
		let list = params.iter().zip(args.iter());
		for (_, (name, o)) in list.enumerate() {
			scoped_env.set(name, o.copy());
		}
		let runtime: Runtime = Runtime::new_from_env(Rc::new(RefCell::new(scoped_env)));
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
		ast.statements = body.0;

		let object = match runtime.run(ast, module) {
			Ok(object) => object,
			Err(mut exception) => {
				exception.push(ExceptionPoint::new(module.clone(), Position::default()));
				return Err(exception);
			}
		};
