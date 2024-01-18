mod cst;

use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub calc);
use cst::*;

#[derive(Debug)]
enum Term {
    Code(String),
    Function {
        name: String,
        args: Vec<Expr>
    }
}

#[derive(Debug)]
struct Expr {
    list: Vec<Term>
}

fn fold_expr(expr: CstExpr) -> Vec<Term> {
    match expr {
        CstExpr::List(bexpr, bterm) => {
            let mut vec = fold_expr(*bexpr);
            vec.push(term_ast(*bterm));
            vec
        }
        CstExpr::Term(bterm) => {
            vec![term_ast(*bterm)]
        }
    }
}

fn expr_ast(expr: CstExpr) -> Expr {
    Expr { list: fold_expr(expr) }
}

fn term_ast(term: CstTerm) -> Term {
    match term {
        CstTerm::Code(string) => Term::Code(string),
        CstTerm::Function(bfunction) => {
            let (string, vec) = function_ast(*bfunction);
            Term::Function{
                name: string,
                args: vec
            }
        }
    }
}

fn function_ast(function: CstFunction) -> (String, Vec<Expr>) {
    match function {
        CstFunction::NoArgs(string) => (string, vec![]),
        CstFunction::WithArgs(string, barglist) => (string, arglist_ast(*barglist))
    }
}

fn arglist_ast(arglist: CstArglist) -> Vec<Expr> {
    match arglist {
        CstArglist::List(barglist, bexpr) => {
            let mut vec = arglist_ast(*barglist);
            vec.push(expr_ast(*bexpr));
            vec
        }
        CstArglist::Expr(bexpr) => vec![expr_ast(*bexpr)]
    }
}

fn replace(format: &str, args: Vec<String>) -> String {
    let mut result = String::from(format);
    #[allow(clippy::needless_range_loop)]
    for i in 0..args.len() {
        let to_replace = String::from("$") + &i.to_string();
        result = result.replace(&to_replace, &args[i])
    }
    result
}

fn term_codegen(term: &Term) -> String {
    match term {
        Term::Code(string) => string.clone(),
        Term::Function { name, args } => {
            let mapping = HashMap::from([
                ("#frac", r"\frac{$0}{$1}"),
                ("#sum", r"\sum_{$0}^{$1}{$2}"),
                ("#bp", r"\left( $0 \right)"),
                ("#bb", r"\left[ $0 \right]"),
                ("#set", r"\left\{ $0 \right\}"),
                ("#partial", r"\frac{\partial $0}{\partial $1}"),
                ("#ang", r"\left\langle $0 \right\rangle"),
                ("#at", r"@")
            ]);

            replace(
                mapping.get(name.as_str()).unwrap(),
                args.iter().map(expr_codegen).collect()
            )
        }
    }
}

fn expr_codegen(expr: &Expr) -> String {
    expr.list.iter().map(term_codegen).collect()
}

fn main() {
    let cst = calc::ExprParser::new().parse(r"
        1 + #bp(#frac(1 @ #sum(i=0 @ n @ #sum(j=0 @ n @ #ang(i, j)))))^{#partial(d^2u @ dx^2)\big|_{x=0}}
    ".trim());

    if let Ok(expr) = cst {
        println!("{:?}", expr);
        let ast = expr_ast(*expr);
        println!("{:?}", ast);
        let code = expr_codegen(&ast);
        println!("{code}");
    }
    else if let Err(lalrpop_util::ParseError::UnrecognizedToken { token, expected }) = cst {
        println!("{:?}", token);
        for e in expected {
            println!("{e}");
        }
    }
    else {
        println!("{:?}", cst);
    }
}