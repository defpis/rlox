use crate::{
    environment::{Environment, Stateful},
    instance::IsInstance,
    interpreter::{InterpretError, Interpreter},
    object::Object,
    stmt::FunctionStmt,
};
use chrono::Utc;
use std::{cell::RefCell, fmt, rc::Rc};

pub trait IsFunction: fmt::Debug + fmt::Display {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpretError>;

    fn bind(
        &self,
        _: Rc<RefCell<dyn IsInstance>>,
    ) -> Result<Rc<RefCell<dyn IsFunction>>, InterpretError> {
        Err(InterpretError::Error("Unreachable error!".to_string()))
    }
}

#[derive(Debug)]
pub struct Clock {}

impl Clock {
    pub fn new() -> Clock {
        Clock {}
    }
}

impl IsFunction for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Object>) -> Result<Object, InterpretError> {
        Ok(Object::Number(Utc::now().timestamp() as f64))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<builtin-fun clock>")
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Rc<FunctionStmt>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl Function {
    pub fn new(
        declaration: Rc<FunctionStmt>,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Function {
        Function {
            declaration,
            closure,
            is_initializer,
        }
    }
}

impl IsFunction for Function {
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

        let value = interpreter.execute_block(&self.declaration.body, environment)?;

        if self.is_initializer {
            return self.closure.borrow().get("this");
        }

        Ok(value)
    }

    fn bind(
        &self,
        instance: Rc<RefCell<dyn IsInstance>>,
    ) -> Result<Rc<RefCell<dyn IsFunction>>, InterpretError> {
        let environment = Environment::new(Some(self.closure.clone()));
        environment
            .borrow_mut()
            .define("this".to_string(), Object::Instance(instance));
        Ok(Rc::new(RefCell::new(Function::new(
            self.declaration.clone(),
            environment,
            self.is_initializer,
        ))))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<fun {}>", self.declaration.name.lexeme)
    }
}
