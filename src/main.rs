extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io;
use std::io::BufRead;
use pest::iterators::Pairs;
use pest::Parser;
use pest::pratt_parser::{Assoc, Op, PrattParser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MathParser;

#[derive(Debug)]
pub enum Expr {
    Value(f64),
    BinaryOperation(BinaryOperation)
}

#[derive(Debug)]
pub struct BinaryOperation {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Operator
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    // FWIW, I think prat means something rude in British English
    let pratt = PrattParser::new()
        .op(Op::infix(Rule::PLUS_SIGN, Assoc::Left) | Op::infix(Rule::MINUS_SIGN, Assoc::Left))
        .op(Op::infix(Rule::STAR_SIGN, Assoc::Left) | Op::infix(Rule::FORWARD_SLASH, Assoc::Left));

    pratt
        .map_primary(|p| match p.as_rule() {
            Rule::number => Expr::Value(p.as_str().parse::<f64>().unwrap()),
            Rule::expr => parse_expr(p.into_inner()),
            rule => unreachable!("Invalid rule encountered: {:?}", rule)
        })
        .map_infix(|left, op, right| {
            let operator = match op.as_rule() {
                Rule::PLUS_SIGN => Operator::Add,
                Rule::MINUS_SIGN => Operator::Subtract,
                Rule::STAR_SIGN => Operator::Multiply,
                Rule::FORWARD_SLASH => Operator::Divide,
                rule => unreachable!("Invalid rule encountered: {:?}", rule)
            };

            Expr::BinaryOperation {
                0: BinaryOperation {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator
                },
            }
        })
        .parse(pairs)
}

fn evaluate_expr(expr: Expr) -> f64 {
    if let Expr::Value(v) = expr {
        return v;
    }

    if let Expr::BinaryOperation(op) = expr {
        let l = evaluate_expr(*op.left);
        let r = evaluate_expr(*op.right);

        return match op.operator {
            Operator::Add => l + r,
            Operator::Subtract => l - r,
            Operator::Multiply => l * r,
            Operator::Divide => l / r,
        };
    };

    unreachable!("Unknown Expr: {:?}", expr);
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let as_str = &line.unwrap_or_else(|e| panic!("{}", e));

        if let Ok(pairs) = MathParser::parse(Rule::expr, as_str) {
            let parsed = parse_expr(pairs);
            println!("{:?}", parsed);
            println!("{:?}", evaluate_expr(parsed));
        } else {
            println!("Invalid input");
            return;
        }
    }
}
