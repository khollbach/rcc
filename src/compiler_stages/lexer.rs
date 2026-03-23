use std::sync::LazyLock;

use anyhow::{Result, bail};
use regex::Regex;

use crate::data_types::token::{Keyword, Token};

pub fn lex(mut s: &str) -> Result<Vec<Token>> {
    let mut out = vec![];

    loop {
        s = s.trim_start();
        if s.is_empty() {
            break;
        }

        let token = take_first_token(&mut s)?;
        out.push(token);
    }

    Ok(out)
}

fn take_first_token(s: &mut &str) -> Result<Token> {
    let (token, token_str) = get_first_token(s)?;
    *s = &s[token_str.len()..];
    Ok(token)
}

fn get_first_token(input: &str) -> Result<(Token, &str)> {
    type ParseFn = fn(&str) -> Result<Token>;

    // (Could we merge all of these into one big regex, to speed it up?)
    static REGEXES: LazyLock<[(Regex, ParseFn); 10]> = LazyLock::new(|| {
        let regexes: [(_, ParseFn); _] = [
            // (Once we have more keywords, might make sense to refactor somehow?)
            // >> The book has a suggestion! We can just match all "words",
            //    and then check for keywords after.
            (r"^int\b", |_| Ok(Token::Keyword(Keyword::Int))),
            (r"^void\b", |_| Ok(Token::Keyword(Keyword::Void))),
            (r"^return\b", |_| Ok(Token::Keyword(Keyword::Return))),
            (r"^[_A-Za-z]\w*", |s| Ok(Token::Ident(s.to_string()))),
            (r"^\d+\b", |s| Ok(Token::Literal(s.parse()?))),
            (r"^\(", |_| Ok(Token::LParen)),
            (r"^\)", |_| Ok(Token::RParen)),
            (r"^\{", |_| Ok(Token::LBrace)),
            (r"^\}", |_| Ok(Token::RBrace)),
            (r"^;", |_| Ok(Token::Semi)),
        ];
        regexes.map(|(r, f)| (Regex::new(r).unwrap(), f))
    });

    for (regex, parse_fn) in &*REGEXES {
        if let Some(token) = regex.find(input) {
            let token = token.as_str();
            return Ok((parse_fn(token)?, token));
        }
    }

    bail!("failed to tokenize. no regex matched {input:?}");
}
