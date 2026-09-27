#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use serde::{Serialize, Deserialize};

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub reference: String,
    pub value: String,
    pub r#type: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub source: String,
    pub target: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub name: String,
    pub connections: Vec<Connection>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchematicData {
    pub components: Vec<Component>,
    pub nets: Vec<Net>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pad {
    pub name: String,
    pub net: String,
    pub x: f64,
    pub y: f64,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PCBBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PCBData {
    pub board_name: String,
    pub footprints: Vec<Footprint>,
    pub traces: Vec<Trace>,
    pub board_bounds: PCBBounds,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BoardSyncRequest {
    pub filename: String,
    pub layer_count: i32,
    pub thermal_metrics: serde_json::Value,
    pub dimensions: serde_json::Value,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DesignCheckError {
    pub r#type: String,
    pub severity: String,
    pub net_name: Option<String>,
    pub details: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DesignCheckReport {
    pub timestamp: String,
    pub error_count: i32,
    pub warning_count: i32,
    pub errors: Vec<DesignCheckError>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy)]
pub struct KiCadState {
    pub schematic: Signal<SchematicData>,
    pub pcb: Signal<PCBData>,
    pub board_sync: Signal<Option<BoardSyncRequest>>,
    pub drc_report: Signal<Option<DesignCheckReport>>,
    pub active_panel: Signal<String>, // "designer" | "library" | "drc" | "chat"
    pub selected_ref: Signal<Option<String>>,
    pub search_query: Signal<String>,
    pub is_loading: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub visible_layers: Signal<std::collections::HashSet<String>>,
}

#[cfg(target_arch = "wasm32")]
impl KiCadState {
    pub fn instantiate_part(&mut self, name: &str, value: &str, type_str: &str, pins: &[(String, String)], layer: &str) {
        let reference = name.split(' ').next().unwrap_or("U").to_string();
        
        // 1. Update Schematic State
        let mut sch = self.schematic.read().clone();
        if sch.components.iter().any(|c| c.reference == reference) {
            *self.error_msg.write() = Some(format!("{} is already instantiated.", reference));
            return;
        }
        
        sch.components.push(Component {
            reference: reference.clone(),
            value: value.to_string(),
            r#type: type_str.to_string(),
        });
        
        for pin in pins {
            let net_name = &pin.1;
            let mut found_net = false;
            for n in &mut sch.nets {
                if &n.name == net_name {
                    n.connections.push(Connection {
                        source: reference.clone(),
                        target: "U1".to_string(), // Connect to MCU
                    });
                    found_net = true;
                    break;
                }
            }
            if !found_net {
                sch.nets.push(Net {
                    name: net_name.clone(),
                    connections: vec![Connection {
                        source: reference.clone(),
                        target: "U1".to_string(),
                    }],
                });
            }
        }
        *self.schematic.write() = sch;

        // 2. Update PCB State
        let mut pcb = self.pcb.read().clone();
        let idx = pcb.footprints.len();
        let px = 40.0 + ((idx * 12) % 60) as f64;
        let py = 40.0 + ((idx * 12) % 60) as f64;

        let pads = pins.iter().enumerate().map(|(i, pin)| {
            Pad {
                name: pin.0.clone(),
                net: pin.1.clone(),
                x: px - 4.0 + (i as f64) * 4.0,
                y: py - 4.0,
            }
        }).collect();

        pcb.footprints.push(Footprint {
            reference: reference.clone(),
            value: value.to_string(),
            x: px,
            y: py,
            orientation: 0.0,
            layer: layer.to_string(),
            pads,
        });

        // Add auto-traces
        for (i, pin) in pins.iter().enumerate() {
            pcb.traces.push(Trace {
                start_x: px - 4.0 + (i as f64) * 4.0,
                start_y: py - 4.0,
                end_x: 60.0, // MCU x
                end_y: 60.0, // MCU y
                net: pin.1.clone(),
                width: if pin.1 == "GND" || pin.1 == "VCC" { 0.6 } else { 0.25 },
                layer: layer.to_string(),
            });
        }
        
        *self.pcb.write() = pcb;
        *self.error_msg.write() = None;
    }
}
