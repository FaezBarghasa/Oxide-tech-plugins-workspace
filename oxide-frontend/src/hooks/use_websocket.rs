use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

#[cfg(target_family = "wasm")]
pub fn use_websocket(url: String, on_message: impl FnMut(String) + Send + Sync + 'static) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{MessageEvent, WebSocket};

    let on_message_shared = Arc::new(Mutex::new(on_message));

    use_effect(move || {
        let on_message_clone = on_message_shared.clone();
        let ws = WebSocket::new(&url).expect("Failed to create WebSocket");
        
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().as_string() {
                if let Ok(mut handler) = on_message_clone.lock() {
                    handler(txt);
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    });
}

#[cfg(not(target_family = "wasm"))]
pub fn use_websocket(url: String, on_message: impl FnMut(String) + Send + Sync + 'static) {
    let on_message_shared = Arc::new(Mutex::new(on_message));
    use_effect(move || {
        let url_clone = url.clone();
        let on_message_clone = on_message_shared.clone();
        std::thread::spawn(move || {
            if let Ok((mut socket, _)) = tungstenite::connect(&url_clone) {
                loop {
                    match socket.read() {
                        Ok(msg) => {
                            if let tungstenite::Message::Text(txt) = msg {
                                if let Ok(mut handler) = on_message_clone.lock() {
                                    handler(txt.to_string());
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
        });
    });
}
