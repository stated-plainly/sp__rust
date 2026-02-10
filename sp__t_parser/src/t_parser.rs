use sp__t_ast::tAST;
use sp__t_token_kind::tTokenKind;

pub trait tParser<K: tTokenKind> {
    type Output : tAST;

    fn parse(token_list: Vec<K>) -> Self::Output;
}
