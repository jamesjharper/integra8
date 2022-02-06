#[macro_export]
macro_rules! src_loc {
    () => {
        $crate::ComponentLocation {
            file_name: std::borrow::Cow::from(file!()),
            column: column!(),
            line: line!(),
        }
    };
}
