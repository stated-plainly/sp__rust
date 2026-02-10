use sp__s_borrower::sBorrower;

pub enum eSourceCode {
    PathToSourceCode(sBorrower<String>),
    SourceCode(sBorrower<String>),
}
