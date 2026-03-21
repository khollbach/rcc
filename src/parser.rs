use crate::{
    ast::{Ast, Expr, FunctionDef, Program, Statement},
    token::{Keyword, Token},
};

use anyhow::{Context, Result, bail, ensure};

pub fn parse(tokens: &[Token]) -> Result<Ast> {
    Parser { tokens }.parse()
}

struct Parser<'a> {
    tokens: &'a [Token],
}

impl Parser<'_> {
    fn parse(mut self) -> Result<Ast> {
        let f = self.parse_function_def()?;
        ensure!(
            self.tokens.is_empty(),
            "trailing tokens after parsing: {:?}",
            self.tokens
        );

        Ok(Ast {
            program: Program::FunctionDef(f),
        })
    }

    fn expect(&mut self, expected_token: Token) -> Result<()> {
        let first_token = self.tokens.get(0);
        if first_token != Some(&expected_token) {
            bail!(
                "failed to parse. expected {:?} got {:?}",
                expected_token,
                first_token,
            );
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }

    fn take_token(&mut self) -> Option<Token> {
        if self.tokens.is_empty() {
            return None;
        }
        let token = self.tokens[0].clone();
        self.tokens = &self.tokens[1..];
        Some(token)
    }

    fn parse_function_def(&mut self) -> Result<FunctionDef> {
        self.expect(Token::Keyword(Keyword::Int))?;
        self.expect(Token::Ident("main".to_string()))?;
        self.expect(Token::LParen)?;
        self.expect(Token::Keyword(Keyword::Void))?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        let ret = self.parse_return_statement()?;

        self.expect(Token::RBrace)?;

        Ok(FunctionDef {
            name: "main".to_string(),
            body: ret,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.expect(Token::Keyword(Keyword::Return))?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Statement::Return(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let value = self.parse_constant()?;
        Ok(Expr::Constant(value))
    }

    fn parse_constant(&mut self) -> Result<i32> {
        let token = self.take_token().context("no more tokens")?;
        let Token::Literal(value) = token else {
            bail!("expected literal constant");
        };
        Ok(value)
    }
}
