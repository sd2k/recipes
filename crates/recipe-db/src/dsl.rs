use diesel::{sql_function, sql_types::*};

sql_function! {
    /// Represents the Pg `LOWER` function used with text.
    fn lower(x: Text) -> Text;
}

/// The return type of `lower(expr)`
pub type Lower<Expr> = lower::HelperType<Expr>;
