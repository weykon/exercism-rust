use std::{collections::HashMap, rc::Rc};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    operation: HashMap<String, WordDefinition>,
}
type WordDefinition = Vec<Rc<Expression>>;

enum Expression {
    Number(Value),
    CustomWord(String),
    Dup,
    Drop,
    Swap,
    Over,
    Add,
    Subtract,
    Multiply,
    Divide,
    WordDefinition(String, WordDefinition),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// 除以0 不允许
    DivisionByZero,
    /// 栈空
    StackUnderflow,
    /// 未知符号
    UnknownWord,
    /// 无效符号
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        unimplemented!()
    }

    pub fn stack(&self) -> &[Value] {
        unimplemented!()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        // 解析这里有人把is_control先把空格和分行各类的格式化后处理，而我觉得没有必要
        let mut chars = input.chars().peekable();
        let mut word = String::new();

        while let Some(&c) = chars.peek() {
            if !c.is_control() && !c.is_whitespace() {
                word.push(c);
            } else {
                if c == ':' {
                    self.push_new_operation();
                } else if c == ';' {
                    self.ending_this_operation();
                } else {
                    if !word.is_empty() {
                        self.token_analysis(&word)?;
                        word.clear();
                    }
                }
            }
            chars.next();
        }

        Ok(())
    }

    fn token_analysis(&mut self, token: &str) -> Result {
        let token = token.to_lowercase();
        let token_str = token.as_str();
        match token_str {
            "+" => self.add(),
            "-" => self.sub(),
            "*" => self.mul(),
            "/" => self.div(),
            "dup" => self.dup(),
            "drop" => self.drop(),
            "swap" => self.swap(),
            "over" => self.over(),
            _ => self.push(token_str),
        }
    }

    fn add(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn sub(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn mul(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn div(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn dup(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn over(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn swap(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn drop(&self) -> std::result::Result<(), Error> {
        todo!()
    }

    fn push(&mut self, token: &str) -> std::result::Result<(), Error> {
        let v = token.parse::<Value>().or(Err(Error::UnknownWord)).unwrap();
        self.stack.push(v);
        Ok(())
    }

    fn push_new_operation(&self) {}

    fn ending_this_operation(&self) {
        todo!()
    }
}
