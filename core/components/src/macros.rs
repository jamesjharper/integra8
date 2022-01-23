#[macro_export]
macro_rules! src_loc {
    () => {
        $crate::ComponentLocation {
            file_name: file!(),
            column: column!(),
            line: line!(),
        }
    };
}
