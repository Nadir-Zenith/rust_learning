use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
mod lexer;
use lexer::Lexer;
use lexer::Token;

#[derive(Clone)]
pub enum Expression {
    Atom(char),
    Operation(char, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut ::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

impl Expression {
    fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0.0)
    }
    #[allow(unused)]
    fn is_asign(&self) -> Option<(char, &Expression)> {
        match self {
            Expression::Atom(_) => return None,
            Expression::Operation(c, operands) => {
                if *c == '=' {
                    let var_name = match operands.first().unwrap() {
                        Expression::Atom(c) => {
                            if *c >= 'a' && *c <= 'z' || *c >= 'A' && *c <= 'Z' {
                                *c
                            } else {
                                panic!("Not a variable name: {}", c)
                            }
                        }
                        _ => unreachable!(),
                    };
                    return Some((var_name, operands.last().unwrap()));
                }
                return None;
            }
        }
    }
    #[allow(unused)]
    fn eval(&self, variables: &HashMap<char, f32>) -> f32 {
        match self {
            Expression::Atom(c) => match c {
                '0'..='9' => return c.to_digit(10).unwrap() as f32,
                'a'..='z' | 'A'..='Z' => *variables
                    .get(c)
                    .expect(&format!("Undefined variable {}", c)),
                _ => unreachable!(),
            },
            Expression::Operation(operator, operands) => {
                let lhs = operands.first().unwrap().eval(variables);
                let rhs = operands.last().unwrap().eval(variables);
                match operator {
                    '+' => return lhs + rhs,
                    '-' => return lhs - rhs,
                    '*' => return lhs * rhs,
                    '/' => return lhs / rhs,
                    '^' => return lhs.powf(rhs),
                    '√' => return lhs.powf(1.0 / (rhs)),
                    op => panic!("Bad operator: {}", op),
                }
            }
        }
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        Token::Op('(') => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(')') => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };
        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }
        lexer.next();
        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }
    lhs
}

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '=' => (0.2, 0.1),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        '^' | '√' => (3.1, 3.0),
        '.' => (4.0, 4.1),
        _ => panic!("bad op: {:?}", op),
    }
}
