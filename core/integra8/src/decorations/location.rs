#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    pub file_name: &'static str,
    pub column: u32,
    pub line: u32,
}

impl SourceLocation {
   pub fn hotlink_text(&self) -> String {
      format!("{}:{}:{}", self.file_name,self.line,self.column)
   }
}


#[macro_export]
macro_rules! current_source_location {
    () => {
      $crate::decorations::SourceLocation {
         file_name: file!(),
         column: column!(),
         line: line!(),
      }
    }
}