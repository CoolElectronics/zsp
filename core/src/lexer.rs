use crate::parser::Constant;
use std::collections::HashMap;

macro_rules! mk {
    ($st:expr,$sym:expr) => {
        ($st.to_string(), $sym)
    };
}
lazy_static! {
    static ref INTERRUPTS: HashMap<char, Option<Symbol>> = HashMap::from([
        ('\n', None),
        (';', None),
        (' ', None),
        ('\u{0017}', None),
        ('(', Some(Symbol::ParenStart)),
        (')', Some(Symbol::ParenEnd)),
        ('{', Some(Symbol::BlockStart)),
        ('}', Some(Symbol::BlockEnd)),
        ('+', Some(Symbol::Op(Op::Plus))),
        ('-', Some(Symbol::Op(Op::Minus))),
        ('*', Some(Symbol::Op(Op::Multiply))),
        ('^', Some(Symbol::Op(Op::Power))),
        ('[', Some(Symbol::IndexStart)),
        (']', Some(Symbol::IndexEnd)),
        (':', Some(Symbol::IndexObject)),
        ('|', Some(Symbol::Lambda)),
        ('@', Some(Symbol::Reference)),
    ]);
    static ref KEYWORDS: HashMap<String, Symbol> = HashMap::from([
        mk!("if", Symbol::If),
        mk!("loop", Symbol::Loop),
        mk!("break", Symbol::Break),
        mk!("return", Symbol::Return),
        mk!("for", Symbol::For),
        mk!("load", Symbol::Load),
        mk!("true", Symbol::Constant(Constant::Bool(true))),
        mk!("false", Symbol::Constant(Constant::Bool(false))),
        mk!("=", Symbol::Assign),
        mk!("else", Symbol::Else),
        mk!("<", Symbol::Op(Op::LessThan)),
        mk!("<=", Symbol::Op(Op::LessThanOrEqualTo)),
        mk!(">", Symbol::Op(Op::GreaterThan)),
        mk!(">=", Symbol::Op(Op::GreaterThanOrEqualTo)),
        mk!("==", Symbol::Op(Op::EqualTo)),
        mk!("!=", Symbol::Op(Op::NotEqualTo)),
        mk!("&&", Symbol::Op(Op::And)),
        mk!("||", Symbol::Op(Op::Or)),
        mk!('!', Symbol::Op(Op::Not)),
        mk!("/", Symbol::Op(Op::Divide)),
    ]);
}
pub fn lex(inp: String) -> Vec<Token> {
    let mut chars: Vec<char> = inp.chars().collect::<Vec<char>>();
    chars.push('\u{0017}');

    let mut idx = 0;
    let mut tokens: Vec<Token> = vec![];
    let mut buf = String::default();

    while idx < chars.len() {
        let mut ch = chars[idx];

        if buf == "//" {
            loop {
                ch = chars[idx];
                if ch == '\n' || ch == '\u{0017}' {
                    buf = String::default();
                    break;
                }
                idx += 1;
            }
        }
        match ch {
            '"' => {
                let mut sbf = String::default();
                let mut depth = 1;
                while depth > 0 {
                    idx += 1;
                    ch = chars[idx];
                    if ch == '"' {
                        depth -= 1;
                    } else {
                        sbf += &ch.to_string();
                    }
                }
                tokens.push(Token {
                    symbol: Symbol::Constant(Constant::String(sbf)),
                    index: idx,
                });
                buf = String::default();
            }
            _ => match INTERRUPTS.get(&ch) {
                Some(opt) => {
                    if buf.len() > 0 {
                        tokens.push(Token {
                            symbol: match KEYWORDS.get(&*buf) {
                                Some(sym) => sym.clone(),
                                None => match buf.parse::<f32>() {
                                    Ok(num) => Symbol::Constant(Constant::Number(num)),
                                    Err(_) => Symbol::Name(buf),
                                },
                            },
                            index: idx,
                        });
                    }
                    match opt {
                        Some(sym) => tokens.push(Token {
                            symbol: sym.clone(),
                            index: idx,
                        }),
                        None => {}
                    }
                    buf = String::default();
                }
                None => buf.push(ch),
            },
        }
        idx += 1;
    }

    tokens
}

#[derive(Debug, Clone)]
pub struct Token {
    pub symbol: Symbol,
    pub index: usize,
}
#[derive(Debug, Clone)]
pub enum Symbol {
    Assign,
    If,
    BlockStart,
    BlockEnd,
    ParenStart,
    ParenEnd,
    Name(String),
    Constant(Constant),
    Op(Op),
    Return,
    Loop,
    Break,
    Else,
    Load,
    For,
    IndexStart,
    IndexEnd,
    IndexObject,
    Lambda,
    Reference,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Not,
    And,
    Or,
}

impl Symbol {
    pub fn display_name(&self) -> String {
        for (k, v) in INTERRUPTS.clone().into_iter() {
            if let Some(s) = v {
                if std::mem::discriminant(&s) == std::mem::discriminant(&self) {
                    return k.to_string();
                }
            }
        }
        for (k, v) in KEYWORDS.clone().into_iter() {
            if std::mem::discriminant(&v) == std::mem::discriminant(&self) {
                return k.to_string();
            }
        }
        return "(Name)".into();
    }
}
