#[macro_export]
macro_rules! src_loc {
    () => {
        $crate::components::ComponentLocation {
            file_name: std::borrow::Cow::from(file!()),
            column: column!(),
            line: line!(),
            path: $crate::components::ComponentPath::from(module_path!()),
        }
    };
}
