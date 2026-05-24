use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Expr {
    Eq { column: String, value: Value },
}
