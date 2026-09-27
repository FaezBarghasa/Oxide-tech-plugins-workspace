use oxide_plugin_altium::ipc2581_bridge::{AltiumBridge, AltiumCOMClient};

#[tokio::test]
async fn test_altium_bridge_import() {
    let bridge = AltiumBridge::new(Some(AltiumCOMClient));
    let result = bridge.import_from_kicad("dummy_kicad_project").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_altium_bridge_export() {
    let bridge = AltiumBridge::new(Some(AltiumCOMClient));
    let result = bridge.export_to_kicad("dummy_altium_project").await;
    assert!(result.is_ok());
}
