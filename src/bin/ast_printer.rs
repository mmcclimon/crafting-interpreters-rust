use lox::expr::{Expr, Literal};
use lox::tools::ast_printer;
use lox::{Token, TokenType as TT};

fn main() {
  let e = Box::new(Expr::Binary(
    Box::new(Expr::Unary(
      Token::new(TT::Minus, 1),
      Box::new(Expr::Literal(Literal::Number(123.0))),
    )),
    Token::new(TT::Star, 1),
    Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
      45.67,
    ))))),
  ));

  ast_printer::print_ast(e);
}
