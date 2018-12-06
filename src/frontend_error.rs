use model::ast::Span;
use codemap::CodeMap;

pub type FrontendResult<T> = Result<T, Vec<FrontendError>>;
pub struct FrontendError {
    pub err: String,  // consider variants with &'static str and owning String
    pub span: Span,
}

pub fn format_errors(codemap: &CodeMap, errors: Vec<FrontendError>) -> String {
    let mut result = String::new();
    for FrontendError { err, span } in errors {
        let msg = codemap.format_message(span, &err);
        result.push_str(&msg);
    }
    result
}
