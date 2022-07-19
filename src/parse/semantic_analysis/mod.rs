#[cfg(test)]
mod test;

use crate::s_expression::{ SExpression, SExpressionRef, List };
use super::syntactic_analysis::Syntax;

pub fn parse(tree: Syntax) -> SExpressionRef {
    match tree {
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => SExpressionRef::number(n),
        Syntax::String(s) => SExpressionRef::string(s),
        Syntax::Symbol(s) => SExpressionRef::symbol(s),
    }
}

fn parse_list(list: Vec<Syntax>) -> SExpressionRef {
    if list.len() == 0 {
        List::nil()
    } else {
        let children = list.into_iter()
            .map(|syn| parse(syn))
            .collect::<Vec<SExpressionRef>>();

        List::from(children)
    }
}
