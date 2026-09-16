pub struct RustParser;

impl RustParser {
    pub fn extract_symbols(source: &str) -> Vec<String> {
        let mut symbols = Vec::new();

        for line in source.lines() {
            let line = line.trim();

            if line.starts_with("pub struct ")
                || line.starts_with("struct ")
                || line.starts_with("pub enum ")
                || line.starts_with("enum ")
                || line.starts_with("pub trait ")
                || line.starts_with("trait ")
                || line.starts_with("impl ")
                || line.starts_with("pub fn ")
                || line.starts_with("fn ")
            {
                symbols.push(line.to_string());
            }
        }

        symbols
    }
}
