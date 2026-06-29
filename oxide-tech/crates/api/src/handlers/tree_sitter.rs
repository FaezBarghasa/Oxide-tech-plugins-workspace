use actix_web::{HttpResponse, web};
use shared::{AstParseRequest, AstNode, AstParseResponse};

pub async fn parse(_req: web::Json<AstParseRequest>) -> HttpResponse {
    let root = AstNode {
        node_type: "source_file".to_string(),
        text: "fn main() {}".to_string(),
        start_line: 0,
        start_column: 0,
        end_line: 1,
        end_column: 0,
        children: vec![
            AstNode {
                node_type: "function".to_string(),
                text: "fn main()".to_string(),
                start_line: 0,
                start_column: 0,
                end_line: 0,
                end_column: 10,
                children: vec![],
            }
        ],
    };

    HttpResponse::Ok().json(AstParseResponse {
        success: true,
        root: Some(root),
        error: None,
    })
}
