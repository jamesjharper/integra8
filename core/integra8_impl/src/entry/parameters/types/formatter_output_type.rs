use syn::parse_quote;
use syn::{Expr, Path};

pub struct OutputFormatterType {
    pub formatter_factory_type: Box<Expr>,
    pub formatter_settings_type: Box<Expr>,
}

pub enum OutputFormatterTypeValue {
    InlineFactoryType { formatter_factory_type: Box<Expr> },
}

impl OutputFormatterTypeValue {
    pub fn render_tokens(self) -> OutputFormatterType {
        match self {
            Self::InlineFactoryType {
                formatter_factory_type,
            } => {
                // Detect user define external types
                // console_output: <USER_DEFINED_TYPE>

                // TODO: Move to common location?
                let integra8_path: Path = parse_quote!(::integra8);

                OutputFormatterType {
                    formatter_settings_type: parse_quote!(< #formatter_factory_type as #integra8_path ::formaters::OutputFormatterFactory>::FormatterParameters),
                    formatter_factory_type: formatter_factory_type,
                }
            }
        }
    }
}
