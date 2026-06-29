use actix_web::{HttpResponse, web};
use shared::{KicadLoadBoardRequest, PcbData, Component, Net, BoardBounds};

pub async fn load_board(_req: web::Json<KicadLoadBoardRequest>) -> HttpResponse {
    let pcb = PcbData {
        board_name: "project.kicad_pcb".to_string(),
        components: vec![
            Component {
                reference: "U1".to_string(),
                value: "STM32H743".to_string(),
                footprint: "LQFP-100".to_string(),
                package: "LQFP".to_string(),
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
                layer: "F.Cu".to_string(),
            }
        ],
        nets: vec![
            Net {
                name: "GND".to_string(),
                pins: vec!["U1.1".to_string(), "U1.50".to_string()],
                net_class: Some("GND".to_string()),
            }
        ],
        traces: vec![],
        board_bounds: BoardBounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 100.0,
            max_y: 100.0,
        },
    };

    HttpResponse::Ok().json(pcb)
}

pub async fn run_drc(_req: web::Json<KicadLoadBoardRequest>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "violations": [],
        "passed": true
    }))
}
