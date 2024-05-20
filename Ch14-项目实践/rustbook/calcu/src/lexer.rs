///TOKEN部分
use self::Token::*;


#[derive(Debug,PartialEq, Clone)]
pub enum Token {
    LPAREN,         // 左括号(
    RPAREN,         // 右括号)
    ADD,            // 加号+
    SUB,            // 减号-
    MUL,            // 乘号*
    DIV,            // 除号/
    CARET,          // 幂次^
    EQUALS,         // 等于号=
    NUMBER(f64),    //浮点数
    SYMBOL(String), //符号，如变量名、函数名
    EOF             // 文件终结符
}

impl Token {
    /* 返回元组 (prec, associativity) 其中0是左结合，1是右结合*/
    pub fn cal_info(&self) -> Option<(usize, usize)> {
        match *self {
            ADD | SUB => Some((10, 0)),
            MUL | DIV => Some((20, 0)),
            CARET => Some((30, 1)),
            _ => { None}
        }
    }
    //判断是否是EOF
    pub fn is_eof(&self) -> bool{
        match self {
            &EOF => true,
            _ => false
        }
    }
}



impl From<Token> for char{
    fn from(value: Token) -> Self {
        match value{
            LPAREN => '(',
            RPAREN => ')',
            ADD => '+',
            SUB => '-',
            MUL => '*',
            DIV => '/' ,
            CARET => '^',
            EQUALS => '=',
            _ => panic!("invalid char")
        }
    }
}




/// Lexer部分
pub struct Lexer {
    pub cur_ch:  char,
    pub pos: usize,
    pub src: String,
    pub eof: bool
}

impl Lexer {
    /// 根据src来初始化一个Lexer
    pub fn new(src: &str) -> Lexer {
        let mut l = Lexer {
            cur_ch: '\0',
            pos: 0,
            src: src.to_string(),
            eof: false
        };
        
        if src.len() == 0 {
            l.eof = true;
        } else {
            l.cur_ch = src.chars().nth(0).unwrap();
        }

        l
    }

    /// 解析下一个token
    pub fn next_token(&mut self) -> Result<Token, String> {
        // 如果解析到了字符串末尾就返回EOF
        if self.eof {
            return Ok(EOF);
        }

        // 先去除空白符
        self.consume_whitespace();
        
        // 进行模式匹配
        match self.cur_ch {
            // 如果是数字的话，就解析数字
            c if c.is_digit(10) => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while (self.cur_ch.is_digit(10) || self.cur_ch == '.') && !self.eof{
                    self.bump();
                    end += 1;
                }
                Ok(NUMBER(self.src[start..end].parse::<f64>().unwrap()))
            }
            //如果是字母的话，就是一个符号（如变量名或者函数名）
            c if c.is_alphabetic() => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while self.cur_ch.is_alphabetic() && !self.eof {
                    self.bump();
                    end += 1;
                }
                Ok(SYMBOL(self.src[start..end].to_string()))
            }
            '(' => {self.bump(); Ok(LPAREN)}
            ')' => {self.bump(); Ok(RPAREN)}
            '+' => {self.bump(); Ok(ADD)}
            '-' => {self.bump(); Ok(SUB)}
            '*' => {self.bump(); Ok(MUL)}
            '/' => {self.bump(); Ok(DIV)}
            '^' => {self.bump(); Ok(CARET)}
            '=' => {self.bump(); Ok(EQUALS)}
            c => { Err(format!("unexpected token {} at position {}", c, self.pos)) }
        }
    }

    // 更改当前cur_ch为下一个字符
    pub fn bump(&mut self) {
        self.pos += 1;
        if self.pos >= self.src.len() {
            self.eof = true;
            return;
        }
        self.cur_ch = self.src.chars().nth(self.pos).unwrap();
    }

    // 去除空白符
    pub fn consume_whitespace(&mut self) {
        while is_whitespace(self.cur_ch) {
            self.bump();
        }
    }
}

pub fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false
    }
}


// impl fmt::Display for Lexer {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.src)
//     }
// }