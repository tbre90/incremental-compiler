use std::rc::Rc;

use super::token::{Token};

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Int(i64),
    Prim {op: Rc<String>, args: Vec<AstNode>},

    Let {
        bindings: Vec<(Rc<String>, AstNode)>,
        body: Box<AstNode> },

    Var { name: Rc<String> },
    Error { msg: Rc<String>, token: Token },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub info: (),
    pub exp: AstNode,
}