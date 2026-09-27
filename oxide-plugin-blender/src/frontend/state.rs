#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use serde::{Serialize, Deserialize};

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VentConfig {
    pub hole_diameter: f64,
    pub spacing: f64,
    pub quantity: i32,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnclosureParams {
    pub dimensions: Dimensions,
    pub wall_thickness: f64,
    pub material: String,
    pub vent_config: VentConfig,
}

#[cfg(target_arch = "wasm32")]
impl Default for EnclosureParams {
    fn default() -> Self {
        Self {
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
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy)]
pub struct CADState {
    pub params: Signal<EnclosureParams>,
    pub history: Signal<Vec<EnclosureParams>>,
    pub history_idx: Signal<usize>,
    pub is_generating: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub is_modal_open: Signal<bool>,
}

#[cfg(target_arch = "wasm32")]
impl CADState {
    pub fn update_params(&mut self, new_params: EnclosureParams) {
        let mut hist = self.history.write();
        let idx = *self.history_idx.read();
        
        hist.truncate(idx + 1);
        hist.push(new_params.clone());
        if hist.len() > 20 {
            hist.remove(0);
        }
        
        *self.history_idx.write() = hist.len() - 1;
        *self.params.write() = new_params;
    }
    
    pub fn undo(&mut self) {
        let idx = *self.history_idx.read();
        if idx > 0 {
            let new_idx = idx - 1;
            *self.history_idx.write() = new_idx;
            *self.params.write() = self.history.read()[new_idx].clone();
        }
    }
    
    pub fn redo(&mut self) {
        let idx = *self.history_idx.read();
        let hist = self.history.read();
        if idx < hist.len() - 1 {
            let new_idx = idx + 1;
            *self.history_idx.write() = new_idx;
            *self.params.write() = hist[new_idx].clone();
        }
    }
}
