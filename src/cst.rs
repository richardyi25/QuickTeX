// what boxes are necessary?

#[derive(Debug)]
pub enum CstExpr {
    List(Box<CstExpr>, Box<CstTerm>),
    Term(Box<CstTerm>)
}

#[derive(Debug)]
pub enum CstTerm {
    Code(String),
    Function(Box<CstFunction>)
}

#[derive(Debug)]
pub enum CstFunction {
    NoArgs(String),
    WithArgs(String, Box<CstArglist>)
}

#[derive(Debug)]
pub enum CstArglist {
    List(Box<CstArglist>, Box<CstExpr>),
    Expr(Box<CstExpr>)
}
