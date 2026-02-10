use sp__e_source_code::eSourceCode;
use sp__s_token::sToken;
use sp__t_error::tError;
use sp__t_token_kind::tTokenKind;

pub trait tLexer<K: tTokenKind> {
    fn tokenise(&mut self, source_code: eSourceCode) -> Result<Vec<sToken<K>>, Box<dyn tError>>;
}
