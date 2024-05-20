//! AST部分
//! 

use std::collections::HashMap;
use crate::lexer::Token::{self,*};
use crate::lexer::Lexer;


///AST节点trait，传入符号表可以求值
pub trait Node {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64>;
}

/// 数字
pub struct Num {
    pub num: f64
}

impl Node for Num {
    fn eval(&self, _env: &mut HashMap<String, f64>) -> Option<f64> {
        Some(self.num)
    }
}

///ADD节点
pub struct Add {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Add {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let left_val = self.left.eval(env)?;
        let right_val = self.right.eval(env)?;
        Some(left_val + right_val)
    }
}

pub struct Sub {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Sub {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let left_val = self.left.eval(env)?;
        let right_val = self.right.eval(env)?;
        Some(left_val - right_val)
    }
}

pub struct Mul {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Mul {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {        
        let left_val = self.left.eval(env)?;
        let right_val = self.right.eval(env)?;
        Some(left_val * right_val)
    }
}


pub struct Div {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Div {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let left_val = self.left.eval(env)?;
        let right_val = self.right.eval(env)?;
        Some(left_val / right_val)
    }
}

pub struct Pow {
    pub base: Box<dyn Node>,
    pub exponent: Box<dyn Node>
}

impl Node for Pow {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.base.eval(env) {
            Some(b) => {
                match self.exponent.eval(env) {
                    Some(e) => Some(b.powf(e)),
                    None => None
                }
            }
            None => None
        }
    }
}


pub struct Sqrt {
    pub arg: Box<dyn Node>
}
impl Node for Sqrt {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.arg.eval(env) {
            Some(x) => Some(x.sqrt()),
            None => None
        }
    }
}

pub struct Print {
    pub arg: Box<dyn Node>
}

impl Node for Print {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let res = self.arg.eval(env);
        match res {
            Some(x) => println!("{}", x),
            None => {}
        }
        res
    }
}

pub struct Var {
    pub name: String
}

impl Node for Var {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match env.get(&(self.name)[..]) {
            Some(r) => Some(*r),
            None => None
        }
    }
}

pub struct Assignment {
    pub name: String,
    pub value: Box<dyn Node>
}

impl Node for Assignment {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let val = self.value.eval(env);
        match val {
            Some(x) => { env.insert(self.name.clone(), x); }
            None => {}
        }
        val
    }
}



///Parser
pub struct Parser {
    pub current: Token,
    pub lexer: Lexer,
    pub peeked: Option<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let l = Lexer::new(input);
        let p = Parser {
            current: EOF,
            peeked: None,
            lexer: l
        };
        p
    }

    pub fn parse(&mut self) -> Result<Box<dyn Node>, String> {
        self.expr(1)
    }

    pub fn expr(&mut self, prec: usize) -> Result<Box<dyn Node>, String> {
        let mut lhs = self.atom()?;
        let mut rhs;
        loop {
            let curr = self.peek_token()?;
            //println!("{:?}", curr);
            if curr.is_eof() {
                //println!("breaking out of expr loop");
                break;
            }
            let info_tuple = curr.cal_info();
            if info_tuple.is_none() {
                break;
            }
            let (op_prec, op_assoc) = info_tuple.unwrap();
            if op_prec < prec {
                break;
            }
            self.next_token()?;
            match op_assoc {
                0 => {
                    rhs = self.expr(op_prec + 1)?;
                }
                _  => {
                    rhs = self.expr(op_prec)?;
                }
            }
            lhs = self.op(curr, lhs, rhs);

        }
        Ok(lhs)
    }

    pub fn atom(&mut self) -> Result<Box<dyn Node>, String> {

        match self.peek_token()? {
            EOF => { Ok(Box::new( Num {num: 0f64})) }
            LPAREN => {
                self.expect('(')?;
                let e = self.expr(1)?;
                self.expect(')')?;
                Ok(e)
            }
            NUMBER(val) => {
                self.next_token()?;
                Ok(Box::new( Num { num: val }))
            }
            SYMBOL(val) => {
                //only allow math functions for now, no variables
                self.next_token()?;
                match self.peek_token()? {
                    LPAREN => {
                        self.expect('(')?;
                        let e = self.expr(1)?;
                        self.expect(')')?;
                        Ok(self.function(val,e))
                    }
                    SYMBOL(name) => {
                        match &val[..] {
                            "let" => {
                                self.next_token()?;
                                self.expect('=')?;
                                let expr = self.expr(1)?;
                                Ok(Box::new( Assignment { name: name, value: expr}))
                            }
                            _ => {
                                Err("Error: two consecutive symbols".to_string())
                            }
                        }
                   }
                   _ => {
                       Ok(Box::new( Var { name: val }))
                   }
                }
            }
            a => {
                Err(format!("unrecognized atom: {:?}", a))
            }
        }
    }

    //构建二元操作对应的ast
    pub fn op (&self, op: Token, lhs: Box<dyn Node>, rhs: Box<dyn Node>)
            -> Box<dyn Node> {
        match op {
            ADD => {
                Box::new( Add {
                    left: lhs,
                    right: rhs
                })
            }
            SUB => {
                Box::new( Sub {
                    left: lhs,
                    right: rhs
                })
            }
            MUL => {
                Box::new( Mul {
                    left: lhs,
                    right: rhs
                })
            }
            DIV => {
                Box::new( Div {
                    left: lhs,
                    right: rhs
                })
            }
            CARET => {
                Box::new( Pow {
                    base: lhs,
                    exponent: rhs
                })
            }
            o => {
                panic!("unrecognized op: {:?}", o);
            }
        }
    }

    //构建函数ast节点
    pub fn function<'a>(&'a self, op: String, arg: Box<dyn Node>) -> Box<dyn Node> {
        match &op[..] {
            "sqrt" | "SQRT" => {
                Box::new( Sqrt {
                    arg: arg
                })
            }
            "print" => {
                Box::new( Print {
                    arg: arg
                })
            }
            _ => {
                panic!("unrecognized function!");
            }
        }
    }
}

impl Parser {
    /// 代表期待下一个token是tok
    pub fn expect(&mut self, tok: char) -> Result<(), String> {
        self.next_token()?;
        let cur_ch: char = self.current.clone().into();
        if cur_ch != tok {
            return Err(format!("expected {:?} but found {}", tok, cur_ch));
        }
        Ok(())
    }

    /// peek token是当前未处理的最靠前的token
    /// 如果是none就获取下一个token，否则直接返回peeked
    pub fn peek_token(&mut self) -> Result<Token, String> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next_token()?);
        }
        Ok(self.peeked.clone().unwrap())
    }

    /// 更新当前token cur
    pub fn next_token(&mut self) -> Result<(), String> {
        match self.peeked {
            Some(ref mut pk) => {
                self.current = pk.clone();
            }
            None => {
                self.current = self.lexer.next_token()?;
            }
        }
        self.peeked = None;
        Ok(())
    }
}