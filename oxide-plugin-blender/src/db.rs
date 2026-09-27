#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::engine::local::{Db, SurrealKv};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::Surreal;
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::types::SurrealValue;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct VentConfig {
    pub hole_diameter: f64,
    pub spacing: f64,
    pub quantity: i32,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct EnclosureParams {
    pub dimensions: Dimensions,
    pub wall_thickness: f64,
    pub material: String,
    pub vent_config: VentConfig,
}

#[cfg(not(target_arch = "wasm32"))]
pub static DB: std::sync::LazyLock<Surreal<Db>> = std::sync::LazyLock::new(Surreal::init);

#[cfg(not(target_arch = "wasm32"))]
pub async fn init() -> surrealdb::Result<()> {
    DB.connect::<SurrealKv>("oxide_blender.db").await?;
    DB.use_ns("oxide").use_db("blender").await?;
    
    let mut response = DB.query("SELECT * FROM cad_parameters:default").await?;
    let params: Option<EnclosureParams> = response.take(0)?;
    if params.is_none() {
        let default_params = EnclosureParams {
            dimensions: Dimensions {
                width: 100.0,
                height: 80.0,
                depth: 60.0,
            },
            wall_thickness: 2.5,
            material: "pla".to_string(),
            vent_config: VentConfig {
                hole_diameter: 3.0,
                spacing: 5.0,
                quantity: 12,
            },
        };
        let mut create_resp = DB.query("CREATE cad_parameters:default CONTENT $param")
            .bind(("param", default_params))
            .await?;
        let _: Option<EnclosureParams> = create_resp.take(0)?;
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_parameters() -> surrealdb::Result<EnclosureParams> {
    let mut response = DB.query("SELECT * FROM cad_parameters:default").await?;
    let params: Option<EnclosureParams> = response.take(0)?;
    Ok(params.unwrap_or_else(|| EnclosureParams {
        dimensions: Dimensions {
            width: 100.0,
            height: 80.0,
            depth: 60.0,
        },
        wall_thickness: 2.5,
        material: "pla".to_string(),
        vent_config: VentConfig {
            hole_diameter: 3.0,
            spacing: 5.0,
            quantity: 12,
        },
    }))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_parameters(new_params: EnclosureParams) -> surrealdb::Result<EnclosureParams> {
    let mut response = DB.query("UPDATE cad_parameters:default CONTENT $param")
        .bind(("param", new_params))
        .await?;
    let updated: Option<EnclosureParams> = response.take(0)?;
    Ok(updated.expect("Failed to update parameters"))
}
