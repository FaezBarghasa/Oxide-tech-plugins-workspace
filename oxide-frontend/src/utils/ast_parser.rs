#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct AstNode {
    pub name: String,
    pub kind: String,
    pub line: usize,
    pub children: Vec<AstNode>,
}

pub fn parse_rust_ast(code: &str) -> Vec<AstNode> {
    let mut nodes = Vec::new();
    for (idx, line) in code.lines().enumerate() {
        let line_num = idx + 1;
        let line_trimmed = line.trim();
        if line_trimmed.starts_with("pub fn ") || line_trimmed.starts_with("fn ") {
            let start_idx = if line_trimmed.starts_with("pub ") { 7 } else { 3 };
            let name_part = &line_trimmed[start_idx..];
            let clean_name = name_part.split('(').next().unwrap_or(name_part).trim();
            nodes.push(AstNode {
                name: clean_name.to_string(),
                kind: "Function".to_string(),
                line: line_num,
                children: vec![],
            });
        } else if line_trimmed.starts_with("pub struct ") || line_trimmed.starts_with("struct ") {
            let start_idx = if line_trimmed.starts_with("pub ") { 11 } else { 7 };
            let name_part = &line_trimmed[start_idx..];
            let clean_name = name_part.split('{').next().unwrap_or(name_part).trim();
            nodes.push(AstNode {
                name: clean_name.to_string(),
                kind: "Struct".to_string(),
                line: line_num,
                children: vec![],
            });
        } else if line_trimmed.starts_with("pub enum ") || line_trimmed.starts_with("enum ") {
            let start_idx = if line_trimmed.starts_with("pub ") { 9 } else { 5 };
            let name_part = &line_trimmed[start_idx..];
            let clean_name = name_part.split('{').next().unwrap_or(name_part).trim();
            nodes.push(AstNode {
                name: clean_name.to_string(),
                kind: "Enum".to_string(),
                line: line_num,
                children: vec![],
            });
        } else if line_trimmed.starts_with("pub trait ") || line_trimmed.starts_with("trait ") {
            let start_idx = if line_trimmed.starts_with("pub ") { 10 } else { 6 };
            let name_part = &line_trimmed[start_idx..];
            let clean_name = name_part.split('{').next().unwrap_or(name_part).trim();
            nodes.push(AstNode {
                name: clean_name.to_string(),
                kind: "Trait".to_string(),
                line: line_num,
                children: vec![],
            });
        } else if line_trimmed.starts_with("impl ") {
            let name_part = &line_trimmed[5..];
            let clean_name = name_part.split('<').next().unwrap_or(name_part).split('{').next().unwrap_or(name_part).trim();
            nodes.push(AstNode {
                name: format!("impl {}", clean_name),
                kind: "ImplBlock".to_string(),
                line: line_num,
                children: vec![],
            });
        }
    }
    nodes
}
