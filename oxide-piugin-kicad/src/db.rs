#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::engine::local::{Db, SurrealKv};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::Surreal;
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::types::SurrealValue;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub reference: String,
    pub value: String,
    pub r#type: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub source: String,
    pub target: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub name: String,
    pub connections: Vec<Connection>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct SchematicData {
    pub components: Vec<Component>,
    pub nets: Vec<Net>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Pad {
    pub name: String,
    pub net: String,
    pub x: f64,
    pub y: f64,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Footprint {
    pub reference: String,
    pub value: String,
    pub x: f64,
    pub y: f64,
    pub orientation: f64,
    pub layer: String,
    pub pads: Vec<Pad>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub net: String,
    pub width: f64,
    pub layer: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct PCBBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct PCBData {
    pub board_name: String,
    pub footprints: Vec<Footprint>,
    pub traces: Vec<Trace>,
    pub board_bounds: PCBBounds,
}

#[cfg(not(target_arch = "wasm32"))]
pub static DB: std::sync::LazyLock<Surreal<Db>> = std::sync::LazyLock::new(Surreal::init);

#[cfg(not(target_arch = "wasm32"))]
pub async fn init() -> surrealdb::Result<()> {
    DB.connect::<SurrealKv>("oxide_kicad.db").await?;
    DB.use_ns("oxide").use_db("kicad").await?;
    
    // Initialize SchematicData
    let mut sch_response = DB.query("SELECT * FROM schematic_data:default").await?;
    let sch: Option<SchematicData> = sch_response.take(0)?;
    if sch.is_none() {
        let default_sch = SchematicData {
            components: vec![Component {
                reference: "U1".to_string(),
                value: "STM32F405RGT6".to_string(),
                r#type: "MCU".to_string(),
            }],
            nets: vec![],
        };
        let mut create_resp = DB.query("CREATE schematic_data:default CONTENT $sch")
            .bind(("sch", default_sch))
            .await?;
        let _: Option<SchematicData> = create_resp.take(0)?;
    }

    // Initialize PCBData
    let mut pcb_response = DB.query("SELECT * FROM pcb_data:default").await?;
    let pcb: Option<PCBData> = pcb_response.take(0)?;
    if pcb.is_none() {
        let default_pcb = PCBData {
            board_name: "STM32_Breakout.kicad_pcb".to_string(),
            footprints: vec![Footprint {
                reference: "U1".to_string(),
                value: "STM32F405RGT6".to_string(),
                x: 60.0,
                y: 60.0,
                orientation: 0.0,
                layer: "F.Cu".to_string(),
                pads: vec![
                    Pad { name: "GND".to_string(), net: "GND".to_string(), x: 56.0, y: 56.0 },
                    Pad { name: "VCC".to_string(), net: "VCC".to_string(), x: 64.0, y: 56.0 },
                ],
            }],
            traces: vec![],
            board_bounds: PCBBounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 120.0,
                max_y: 120.0,
            },
        };
        let mut create_resp = DB.query("CREATE pcb_data:default CONTENT $pcb")
            .bind(("pcb", default_pcb))
            .await?;
        let _: Option<PCBData> = create_resp.take(0)?;
    }
    
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_schematic() -> surrealdb::Result<SchematicData> {
    let mut response = DB.query("SELECT * FROM schematic_data:default").await?;
    let sch: Option<SchematicData> = response.take(0)?;
    Ok(sch.unwrap_or_else(|| SchematicData {
        components: vec![Component {
            reference: "U1".to_string(),
            value: "STM32F405RGT6".to_string(),
            r#type: "MCU".to_string(),
        }],
        nets: vec![],
    }))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_schematic(new_sch: SchematicData) -> surrealdb::Result<SchematicData> {
    let mut response = DB.query("UPDATE schematic_data:default CONTENT $sch")
        .bind(("sch", new_sch))
        .await?;
    let updated: Option<SchematicData> = response.take(0)?;
    Ok(updated.expect("Failed to update schematic"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_pcb() -> surrealdb::Result<PCBData> {
    let mut response = DB.query("SELECT * FROM pcb_data:default").await?;
    let pcb: Option<PCBData> = response.take(0)?;
    Ok(pcb.unwrap_or_else(|| PCBData {
        board_name: "STM32_Breakout.kicad_pcb".to_string(),
        footprints: vec![Footprint {
            reference: "U1".to_string(),
            value: "STM32F405RGT6".to_string(),
            x: 60.0,
            y: 60.0,
            orientation: 0.0,
            layer: "F.Cu".to_string(),
            pads: vec![
                Pad { name: "GND".to_string(), net: "GND".to_string(), x: 56.0, y: 56.0 },
                Pad { name: "VCC".to_string(), net: "VCC".to_string(), x: 64.0, y: 56.0 },
            ],
        }],
        traces: vec![],
        board_bounds: PCBBounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 120.0,
            max_y: 120.0,
        },
    }))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_pcb(new_pcb: PCBData) -> surrealdb::Result<PCBData> {
    let mut response = DB.query("UPDATE pcb_data:default CONTENT $pcb")
        .bind(("pcb", new_pcb))
        .await?;
    let updated: Option<PCBData> = response.take(0)?;
    Ok(updated.expect("Failed to update PCB"))
}
