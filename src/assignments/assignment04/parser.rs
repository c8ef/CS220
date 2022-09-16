//! Parser.

use self::inner::*;
use super::syntax::*;
use anyhow::Result;
use pest::{
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    let pairs = SyntaxParser::parse(Rule::command, line)?;
    match pairs.peek().unwrap().as_rule() {
        Rule::expr => Ok(Command {
            variable: None,
            expression: turn_pairs_into_expressions(pairs.peek().unwrap().into_inner()),
        }),
        _ => Ok(Command {
            variable: Some(pairs.peek().unwrap().as_str().to_string()),
            expression: turn_pairs_into_expressions(pairs.clone().nth(1).unwrap().into_inner()),
        }),
    }
}

fn turn_pairs_into_expressions(line: Pairs<'_, Rule>) -> Expression {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    let infix = |lhs: Expression, op: Pair<'_, Rule>, rhs: Expression| match op.as_rule() {
        Rule::add => Expression::BinOp {
            op: BinOp::Add,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Rule::subtract => Expression::BinOp {
            op: BinOp::Subtract,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Rule::multiply => Expression::BinOp {
            op: BinOp::Multiply,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Rule::divide => Expression::BinOp {
            op: BinOp::Divide,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        Rule::power => Expression::BinOp {
            op: BinOp::Power,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        _ => unreachable!(),
    };

    let primary = |pair: Pair<'_, Rule>| match pair.as_rule() {
        Rule::num => Expression::Num(pair.as_str().parse::<f64>().unwrap()),
        Rule::var => Expression::Variable(pair.as_str().to_string()),
        Rule::expr => turn_pairs_into_expressions(line.peek().unwrap().into_inner()),
        _ => unreachable!(),
    };

    climber.climb(line.clone(), primary, infix)
}
