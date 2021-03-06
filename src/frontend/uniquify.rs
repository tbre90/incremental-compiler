/*
    make it so that variables are numbered depending on the scope they're defined in
    this allows variable shadowing
*/

use std::collections::HashMap;
use std::rc::Rc;

use super::ast::{AstNode, Program};

fn uniquify_exp(environments: &mut Vec<HashMap<Rc<String>, Rc<String>>>, e: AstNode) -> AstNode {
    match e {
        AstNode::Int(n) => AstNode::Int(n),

        AstNode::Var { name } => {

            let mut new_name = name.clone();

            let last = environments.len();

            for i in (0..last).rev() {
                if environments[i].contains_key(&name) {
                    new_name = environments[i][&name].clone();
                    break;
                }
            }

            AstNode::Var {
                name: new_name
            }
        },

        AstNode::Prim { op, mut args } => {
            for i in 0..args.len() {
                let new_arg_expr = uniquify_exp(environments, args[i].clone());
                args[i] = new_arg_expr;
            }

            AstNode::Prim {
                op: op,
                args: args
            }
        },

        AstNode::Let { bindings, body, } => {

            environments.push(HashMap::new());

            let last = environments.len()-1;

            let mut uniq_bindings: Vec<(Rc<String>, AstNode)> = Vec::new();

            for binding in bindings {

                let current_env = environments.get_mut(last).unwrap();

                let the_var = binding.0;
                let the_value = binding.1;

                //let new_name = the_var.clone() + "." + &(last+1).to_string();
                let new_name = Rc::new((*the_var).clone() + "." + &(last+1).to_string());

                current_env.insert( the_var, new_name.clone() );

                let unq_value = uniquify_exp(environments, the_value);

                uniq_bindings.push((new_name, unq_value));
            }

            let unq_body = uniquify_exp(environments, *body);

            environments.pop();

            AstNode::Let {
                bindings: uniq_bindings,
                body: Box::new(unq_body)
            }
        },

        AstNode::Error { msg, token } => {
            AstNode::Error { msg: msg, token: token }
        },
    }
}

pub fn uniquify_program(p: Program) -> Program {

    let env =
        &mut Vec::<HashMap<Rc<String>, Rc<String>>>::new();

    Program {
        info: p.info,
        exp: uniquify_exp(env, p.exp),
    }
}