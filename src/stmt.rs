use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
  Empty,
  Block(Vec<Stmt>),
  Expression(Box<Expr>),
  If(Box<Expr>, Box<Stmt>, Box<Stmt>),
  Print(Box<Expr>),
  Var(String, Box<Expr>), // maybe instead, Option<Expr>
}
