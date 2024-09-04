use crate::{
    callable::Callable,
    environment::Environment,
    interpreter::{InterpretError, Interpreter},
    object::Object,
    stmt::FunctionStmt,
};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug)]
pub struct Function {
    declaration: FunctionStmt,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: FunctionStmt, closure: Rc<RefCell<Environment>>) -> Function {
        Function {
            declaration,
            closure,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpretError> {
        let environment = Environment::new(Some(self.closure.clone()));

        for i in 0..self.declaration.params.len() {
            environment.borrow_mut().define(
                self.declaration.params.get(i).unwrap().lexeme.clone(),
                arguments.get(i).unwrap().clone(),
            )
        }

        interpreter.execute_block(&self.declaration.body, environment)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<fun {}>", self.declaration.name.lexeme)
    }
}
