


use logos::{Logos, Skip, SpannedIter};



use crate::ast::{Direction, SignalMode};

#[derive(Debug, Logos, PartialEq, Clone)] //TODO: can I not clone?
#[logos(skip r"[ \n\r\t\f]+", extras = ())] // Ignore this regex pattern between tokens

pub enum Token {
    // Key words
    #[token("spec")]
    Spec,
    #[token("impl")]
    Impl,
    #[token("signal")]
    Signal,
    #[token("sync", |_| SignalMode::Sync)]
    #[token("comb", |_| SignalMode::Comb)]
    LogMode(SignalMode),


        //#[token("mod")]
    //Mod,

    //#[token("fn")]
    //Func,
    //#[token("if")]
    //If,
    //#[token("elsif")]
    //Elsif,
    //#[token("else")]
    //Else,
    ////#[token("then")] //maybe force {} instead of then?
    ////Then,
    //#[token("->")]
    //Arrow,
    //#[token("gate")]
    //Gate,
    //#[token("wait")]
    //Wait,
    //#[token("use")]
    //Use,
    //#[token("const")]
    //Const,
    //#[token("var")]
    //Var,

    #[regex("[a-zA-Z_]+[a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9]+", |x| x.slice().parse::<usize>().unwrap())]
    IntLiteral(usize),

    #[token("logic")]
    //#[token("clock", |_| IOType::Clock)]
    //#[token("integer", |_| IOType::Integer)]
    Logic,

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),
    //#[regex(r#""(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'"#)]
    //StringLiteral,

    // Direction operators
    #[token("->", |_| Direction::In)]
    #[token("<-", |_| Direction::Out)]
    #[token("<>", |_| Direction::InOut)]
    Dir(Direction),

    // Arithmetic operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulus,

    // Assignment operators
    #[token("=")]
    Assign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,

    // Comparison operators
    #[token("==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">=")]
    GreaterThanOrEqual,

    // Logical operators
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("!")]
    LogicalNot,

    // Bitwise operators
    #[token("&")]
    BitwiseAnd,
    #[token("^")]
    BitwiseXor,
    #[token("~")]
    BitwiseNot,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("|")]
    Pipe,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,

    #[token(";")]
    Semicolon,

    #[regex("--.*", |_| Skip)]
    LineComment,
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", |_| Skip)]
    BlockComment,
    #[token(r"*/", priority = 2)]
    CommentClose,
}

pub fn lexer(src: &str) -> SpannedIter<'_, Token> {
    Token::lexer(src).spanned()
}

