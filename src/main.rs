use std::ops::Range;
use std::{fs, iter::Peekable, path::Path};

use ast::Type::Logic;
use colored::Colorize;
use logos::Span;
use logos::SpannedIter;
use printer::print_module;
pub mod ast;
pub mod lexer;
pub mod printer;

use ast::{DecZone, Generic, Signal, SignalMode, SignalType, Spec, Type};

use lexer::{lexer, Token};
fn main() {
    let base_path = Path::new("F:/lab/language_design/rain/example_src/");
    let filename = "CRC16.rn";

    //let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(base_path.join(&filename)).expect("Failed to read file");

    let mut lex = lexer(src.as_str()).peekable();

    // here we borrow all the module stuff from the previous rain.
    get_imports();

    //parse_tok(&mut lex, Token::Signal, |_| ());
    //let sig = parse_sig(&mut lex, &src);
    // after import declarations a file can be ... (for now just modules that must start with a spec)
    match parse_spec(&mut lex, &src) {
        Ok(spec) => {
            println!("{:#?}", spec);
            let out_path = Path::new("F:/lab/language_design/Rain/example_output/");
            match parse_impl(&spec, &mut lex, &src){
                Ok(decs) => print_module(&out_path, spec, decs).unwrap(),
                Err(error) => error_handler(error, &src, &filename),
            };
            
        }
        Err(error) => error_handler(error, &src, &filename),
    }
}



fn get_imports() {
    return;
    //TODO
}

fn parse_spec<'a>(
    lex: &mut Peekable<SpannedIter<'_, Token>>,
    src: &'a str,
) -> Result<Spec<'a>, ParseError> {
    let mut spec = Spec {
        name: "",
        generics: None,
        io: Vec::new(),
    };
    parse_tok(lex, Token::Spec, |_| ())?;
    spec.name = parse_tok(lex, Token::Ident, |x| get_ident_str(&x, src))?;
    spec.generics = parse_generics(lex, src)?;

    // consume port.
    parse_tok(lex, Token::BraceOpen, |_| ())?;
    loop {
        let io = parse_sig(lex, src, DecZone::Spec)?;
        spec.io.push(io);

        //next could be the end ('}'), a comma and then the end or another io.
        if let Some((Ok(Token::Comma), _)) = lex.peek() {
            lex.next();
        }
        if let Some((Ok(Token::BraceClose), _)) = lex.peek() {
            lex.next();
            break;
        }
    }
    Ok(spec)
}
fn parse_generics<'a>(
    lex: &mut Peekable<SpannedIter<'_, Token>>,
    src: &'a str,
) -> Result<Option<Vec<Generic<'a>>>, ParseError> {
    // next token can be '[' or '{' depending on generics.
    if let Some((Ok(Token::BracketOpen), _)) = lex.peek() {
        lex.next(); //consume brace.
                    // consume generics.
        let mut generics = Vec::new();
        loop {
            let mut gen = Generic {
                name: "",
                typ: "",
                default: None,
            };
            gen.name = parse_tok(lex, Token::Ident, |x| get_ident_str(&x, src))?;
            parse_tok(lex, Token::Colon, |_| ())?;
            gen.typ = parse_tok(lex, Token::Ident, |x| get_ident_str(&x, src))?;
            if let Some((Ok(Token::Assign), _)) = lex.peek() {
                // has default value.
                lex.next();
                //TODO: parse expression.
                if let Some((Ok(Token::IntLiteral(lit)), _)) = lex.next() {
                    gen.default = Some(lit);
                }
            }
            generics.push(gen);

            //next could be the end (']'), a comma and then the end or another generic.
            if let Some((Ok(Token::Comma), _)) = lex.peek() {
                lex.next();
            }
            if let Some((Ok(Token::BracketClose), _)) = lex.peek() {
                lex.next();
                break;
            }
        }
        Ok(Some(generics))
    } else {
        Ok(None)
    }
}

fn parse_range(lex: &mut Peekable<SpannedIter<'_, Token>>) -> Result<Range<usize>, ParseError> {
    let mut start: usize = 0;
    let mut end: usize = 0;
    if let Some((Ok(Token::BracketOpen), _)) = lex.peek() {
        lex.next(); //consume brace.
        if let Some((Ok(Token::IntLiteral(lit)), _)) = lex.peek() {
            end = *lit;
            lex.next();
        } else {
            //    //TODO: errors?
            return Err(ParseError::Empty);
        }
        if let Some((Ok(Token::Colon), _)) = lex.peek() {
            lex.next();
            if let Some((Ok(Token::IntLiteral(lit)), _)) = lex.peek() {
                start = *lit;
                lex.next();
            } else {
                //    //TODO: errors?
                return Err(ParseError::Empty);
            }
        } else {
            end -= 1;
        }
        parse_tok(lex, Token::BracketClose, |_| ())?;
    }
    if start > end {
        Err(ParseError::Empty) //TODO: invalid range error.
    } else {
        Ok(Range {
            start: start,
            end: end,
        })
    }
}

//OK
fn get_ident_str<'a>(span: &Span, src: &'a str) -> &'a str {
    let (line, col) = compute_line_and_column(&src, span.start);
    let line = src.lines().collect::<Vec<&str>>()[line];
    line.get(col..(col + span.len())).unwrap()
}

//OK
fn parse_sig<'a>(
    lex: &mut Peekable<SpannedIter<'_, Token>>,
    src: &'a str,
    zone: DecZone,
) -> Result<Signal<'a>, ParseError> {
    let name = parse_tok(lex, Token::Ident, |x| get_ident_str(&x, src))?;
    parse_tok(lex, Token::Colon, |_| ())?;
    let mut styp: SignalType = SignalType::Internal;
    let mut mode: SignalMode = SignalMode::Comb;
    match lex.peek() {
        Some((op_tok, span)) => match op_tok {
            Ok(token) => match token {
                Token::Dir(_) => {
                    if let Some((Ok(Token::Dir(direction)), _)) = lex.next() {
                        styp = SignalType::Interface(direction);
                    }
                    if zone == DecZone::Impl {
                        todo!("direction given in a impl block. this is only for specs");
                    }
                }
                Token::LogMode(_) => {
                    if let Some((Ok(Token::LogMode(logm)), _)) = lex.next() {
                        mode = logm;
                        styp = SignalType::Internal;
                    }
                    if zone == DecZone::Spec {
                        todo!("mode given in a impl block. this is only for impls");
                    }
                }
                Token::Logic => {
                    if zone == DecZone::Spec {
                        todo!("ios must be given a direction.");
                    }
                }
                _ => return Err(ParseError::BadTok { span: span.clone() }),
            },
            Err(_) => return Err(ParseError::ErrTok { span: span.clone() }),
        },
        None => return Err(ParseError::Empty), // File ended prematurely
    };
    //parse type
    let typ: Type;
    match lex.next() {
        Some((op_tok, span)) => match op_tok {
            Ok(token) => match token {
                Token::Logic => {
                    let range = parse_range(lex)?;
                    typ = Logic { range };
                }
                _ => return Err(ParseError::BadTok { span: span }),
            },
            Err(_) => return Err(ParseError::ErrTok { span: span }),
        },
        None => return Err(ParseError::Empty), // File ended prematurely
    };

    Ok(Signal {
        name: name,
        sig_typ: styp,
        mode: mode,
        typ: typ,
    })
}

fn parse_impl<'a>(
    spec: &Spec<'a>,
    lex: &mut Peekable<SpannedIter<'_, Token>>,
    src: &'a str,
) -> Result<Vec<Signal<'a>>, ParseError> {

    let mut decs = Vec::new(); 
    //parse beningin
    parse_tok(lex, Token::Impl, |_| ())?;
    let name = parse_tok(lex, Token::Ident, |x| get_ident_str(&x, src))?;

    if name != spec.name {
        todo!("mismatched names")
    }
    parse_tok(lex, Token::BraceOpen, |_| ())?;
    while lex.peek() != None {
        match lex.next() {
            Some((tok, span)) => match tok {
                Ok(tok2) => match tok2 {
                    Token::Spec => todo!(),
                    Token::Impl => todo!(),
                    Token::Ident => todo!(),
                    Token::IntLiteral(_) => todo!(),
                    Token::Bool(_) => todo!(),
                    Token::Dir(direction) => todo!(),
                    Token::Plus => todo!(),
                    Token::Minus => todo!(),
                    Token::Multiply => todo!(),
                    Token::Divide => todo!(),
                    Token::Modulus => todo!(),
                    Token::Assign => todo!(),
                    Token::AddAssign => todo!(),
                    Token::SubAssign => todo!(),
                    Token::MulAssign => todo!(),
                    Token::DivAssign => todo!(),
                    Token::ModAssign => todo!(),
                    Token::Equals => todo!(),
                    Token::NotEquals => todo!(),
                    Token::LessThan => todo!(),
                    Token::GreaterThan => todo!(),
                    Token::LessThanOrEqual => todo!(),
                    Token::GreaterThanOrEqual => todo!(),
                    Token::LogicalAnd => todo!(),
                    Token::LogicalOr => todo!(),
                    Token::LogicalNot => todo!(),
                    Token::BitwiseAnd => todo!(),
                    Token::BitwiseXor => todo!(),
                    Token::BitwiseNot => todo!(),
                    Token::LeftShift => todo!(),
                    Token::RightShift => todo!(),
                    Token::BraceOpen => todo!(),
                    Token::BraceClose => break,
                    Token::ParenOpen => todo!(),
                    Token::ParenClose => todo!(),
                    Token::Pipe => todo!(),
                    Token::Colon => todo!(),
                    Token::Comma => todo!(),
                    Token::BracketOpen => todo!(),
                    Token::BracketClose => todo!(),
                    Token::Semicolon => todo!(),
                    Token::LineComment => todo!(),
                    Token::BlockComment => todo!(),
                    Token::CommentClose => todo!(),
                    Token::Signal => {
                        let sig = parse_sig(lex, src, DecZone::Impl)?;
                        parse_tok(lex, Token::Semicolon, |_|())?;
                        decs.push(sig);
                    }
                    Token::LogMode(log_mode) => {
                        break;
                    }
                    Token::Logic => todo!(),
                },
                Err(_) => todo!(),
            },
            None => todo!(),
        }
    }

    //TODO: rember that in rain you can declare anywhere??
    parse_declarations();

    Ok(decs)
}
fn parse_declarations() {}

#[derive(Debug)]
enum ParseError {
    Empty,
    BadTok { span: Range<usize> },
    ErrTok { span: Range<usize> },
}
fn parse_tok<F, R>(
    lex: &mut Peekable<SpannedIter<'_, Token>>,
    expec_token: Token,
    process_tok: F,
) -> Result<R, ParseError>
where
    F: Fn(Range<usize>) -> R,
{
    match lex.next() {
        Some((tok, span)) => {
            match tok {
                Ok(tok) => {
                    if tok == expec_token {
                        Ok(process_tok(span))
                    } else {
                        //unexpected token.
                        Err(ParseError::BadTok { span })
                    }
                }
                Err(_) => {
                    //erroneous token.
                    Err(ParseError::ErrTok { span })
                }
            }
        }
        None => {
            //empty file
            Err(ParseError::Empty)
        }
    }
}

fn error_handler(error: ParseError, src: &str, filename: &str) {
    match error {
        ParseError::Empty => {
            report(
                None,
                MessageType::Error(&format!(
                    "File ended prematurely, {} expected",
                    "spec declaration"
                )),
                src,
                filename,
            );
        }
        ParseError::BadTok { span } => {
            report(
                Some(&span),
                MessageType::Error(&format!("Invalid token, {} expected", "spec declaration")),
                src,
                filename,
            );
        }
        ParseError::ErrTok { span } => {
            report(
                Some(&span),
                MessageType::Error("Invalid character found by lexer"),
                src,
                filename,
            );
        }
    }
}

///////////////////////////////////////////////////////////

enum MessageType<'a> {
    Error(&'a str),
    Warning(&'a str),
    Info(&'a str),
    Suggestion(&'a str),
    Debug(&'a str),
}

//TODO: add syntax highlighting to the output??
//TODO: add tips and hints to the output, tips are specified when the error is found and passed here for formatting.
fn report(span: Option<&Span>, msg: MessageType, src: &str, filename: &str) {
    let message = match msg {
        MessageType::Error(message) => format!("{}: {}", "Error".red(), message).bold(),
        MessageType::Warning(message) => format!("{}: {}", "Warning".yellow(), message).bold(),
        MessageType::Info(message) => format!("{}: {}", "Info".blue(), message).bold(),
        MessageType::Suggestion(message) => format!("{}: {}", "Suggestion".green(), message).bold(),
        MessageType::Debug(message) => format!("{}: {}", "Debug".purple(), message).bold(),
    };

    if let Some(span) = span {
        let (line, col) = compute_line_and_column(&src, span.start);

        println!(
            "{} \n  {} {}:{}:{}",
            message,
            "-->".cyan().bold(),
            filename,
            line + 1,
            col + 1
        );
        let error_line = src.lines().collect::<Vec<&str>>()[line];
        // Print the offending line with line number
        println!("{}", "     |".cyan().bold());
        println!(
            "{:>4} {} {}",
            format!("{}", line + 1).cyan().bold(),
            "|".cyan().bold(),
            error_line
        );

        // Print a marker below the line to highlight the column
        let marker = " ".repeat(col) + "^".repeat(span.len()).as_str();
        println!("{}{}", "     | ".cyan().bold(), marker.yellow());
        println!("{}", "     |".cyan().bold());
        println!();
    } else {
        println!("{} \n  {} {}", message, "-->".cyan().bold(), filename);
    };
}

fn compute_line_and_column(text: &str, position: usize) -> (usize, usize) {
    /*  separate computation.
        let line = src[..tok_start].chars().filter(|&c| c == '\n').count();
        let col = match src[..tok_start].rfind('\n') {
        Some(last_newline) => tok_start - last_newline -1, // Characters since last newline
        None => tok_start + 1,                          // Entire text is a single line
        };
    */

    if position > text.len() {
        panic!("Position is out of bounds");
    }

    // Iterate over characters with indices
    let (line, last_newline) = text
        .chars()
        .enumerate()
        .take(position) // Only consider up to the given position
        .fold((0, 0), |(line, last_newline), (i, c)| {
            if c == '\n' {
                (line + 1, i)
            } else {
                (line, last_newline)
            }
        });

    // Compute the column
    let column = match last_newline {
        0 => position - last_newline,
        _ => position - last_newline - 1, // Entire string is one line
    };

    (line, column)
}
