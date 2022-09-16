//! Calculator.

use std::collections::HashMap;

use anyhow::*;

use super::syntax::{BinOp, Command, Expression};

use std::error::Error;
use std::fmt;
use std::result::Result::Ok;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Calculator's context.
#[derive(Debug, Default, Clone)]
pub struct Context {
    anonymous_counter: usize,
    variables: HashMap<String, f64>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current anonymous variable counter.
    pub fn current_counter(&self) -> usize {
        self.anonymous_counter
    }

    /// Calculates the given expression. (We assume the absence of overflow.)
    pub fn calc_expression(&self, expression: &Expression) -> Result<f64> {
        match expression {
            Expression::Variable(var) => {
                let val = self.variables.get(var);
                match val {
                    Some(val) => Ok(*val),
                    None => Err(anyhow::Error::new(MyError::new("var not in the hashmap"))),
                }
            }
            Expression::Num(num) => Ok(*num),
            Expression::BinOp { op, lhs, rhs } => match op {
                BinOp::Add => Ok(self.calc_expression(lhs)? + self.calc_expression(rhs)?),
                BinOp::Subtract => Ok(self.calc_expression(lhs)? - self.calc_expression(rhs)?),
                BinOp::Multiply => Ok(self.calc_expression(lhs)? * self.calc_expression(rhs)?),
                BinOp::Divide => {
                    if self.calc_expression(rhs)? == 0_f64 {
                        Err(anyhow::Error::new(MyError::new("cannot divide 0")))
                    } else {
                        Ok(self.calc_expression(lhs)? / self.calc_expression(rhs)?)
                    }
                }
                BinOp::Power => Ok(self.calc_expression(lhs)?.powf(self.calc_expression(rhs)?)),
            },
        }
    }

    /// Calculates the given command. (We assume the absence of overflow.)
    ///
    /// If there is no variable lhs in the command (i.e. `command.variable = None`), its value should be stored at `$0`, `$1`, `$2`, ... respectively.
    ///
    /// # Example
    ///
    /// After calculating commad `3 + 5` => Context's variables = `{($0,8)}`
    ///
    /// After calculating commad `v = 3 - 2` => Context's variables = `{($0,8),(v,1))}`
    ///
    /// After calculating commad `3 ^ 2` => Context's variables = `{($0,8),(v,1),($1,9)}`
    pub fn calc_command(&mut self, command: &Command) -> Result<(String, f64)> {
        let key = match command.variable.clone() {
            Some(name) => name,
            None => {
                let name = format!("${}", self.anonymous_counter);
                self.anonymous_counter += 1;
                name
            }
        };
        let value = self.calc_expression(&command.expression)?;
        let _out = self.variables.insert(key.clone(), value);
        Ok((key, value))
    }
}
