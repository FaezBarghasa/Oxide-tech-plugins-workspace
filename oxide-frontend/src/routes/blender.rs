use dioxus::prelude::*;
use dioxus::document::eval;
use crate::state::*;

#[component]
pub fn BlenderViewer() -> Element {
    let mut params = use_signal(EnclosureParams::default);

    use_effect(move || {
        let p = params.read().clone();
        let js = format!(
            r#"
            let params = {};
            
            let init = () => {{
                let canvas = document.getElementById('threejs-canvas');
                if (!canvas) return;
                
                let rect = canvas.getBoundingClientRect();
                let width = rect.width || 400;
                let height = rect.height || 300;
                
                if (window.blenderScene) {{
                    updateMesh();
                    return;
                }}
                
                let THREE = window.THREE;
                let scene = new THREE.Scene();
                scene.background = new THREE.Color(0x0f172a);
                window.blenderScene = scene;
                
                let camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
                camera.position.set(12, 12, 12);
                camera.lookAt(0, 0, 0);
                window.blenderCamera = camera;
                
                let renderer = new THREE.WebGLRenderer({{ canvas: canvas, antialias: true }});
                renderer.setSize(width, height);
                renderer.shadowMap.enabled = true;
                window.blenderRenderer = renderer;
                
                let gridHelper = new THREE.GridHelper(20, 20, 0x334155, 0x1e293b);
                scene.add(gridHelper);
                
                let ambient = new THREE.AmbientLight(0xffffff, 0.4);
                scene.add(ambient);
                
                let dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
                dirLight.position.set(10, 20, 10);
                scene.add(dirLight);
                
                let group = new THREE.Group();
                scene.add(group);
                window.blenderGroup = group;
                
                let resizeObserver = new ResizeObserver(() => {{
                    let r = canvas.getBoundingClientRect();
                    camera.aspect = r.width / r.height;
                    camera.updateProjectionMatrix();
                    renderer.setSize(r.width, r.height);
                }});
                resizeObserver.observe(canvas);
                
                let animate = () => {{
                    requestAnimationFrame(animate);
                    group.rotation.y += 0.003;
                    renderer.render(scene, camera);
                }};
                animate();
                
                updateMesh();
            }};
            
            let updateMesh = () => {{
                let group = window.blenderGroup;
                if (!group) return;
                
                while(group.children.length > 0){{ 
                    group.remove(group.children[0]); 
                }}
                
                let THREE = window.THREE;
                let scale = 0.05;
                let w = params.dimensions.width * scale;
                let h = params.dimensions.height * scale;
                let d = params.dimensions.depth * scale;
                let t = params.wallThickness * scale;
                
                let matColor = 0xcccccc;
                let metalness = 0.1;
                if (params.material === 'abs') matColor = 0x334155;
                if (params.material === 'petg') matColor = 0x0ea5e9;
                if (params.material === 'aluminum') {{
                    matColor = 0x94a3b8;
                    metalness = 0.85;
                }}
                
                let material = new THREE.MeshStandardMaterial({{
                    color: matColor,
                    roughness: 0.2,
                    metalness: metalness,
                    transparent: true,
                    opacity: 0.85
                }});
                
                let outerGeo = new THREE.BoxGeometry(w, h, d);
                let outerMesh = new THREE.Mesh(outerGeo, material);
                group.add(outerMesh);
                
                let innerGeo = new THREE.BoxGeometry(w - t*2, h - t*2, d - t*2);
                let innerMat = new THREE.MeshBasicMaterial({{ color: 0x38bdf8, wireframe: true, opacity: 0.3, transparent: true }});
                let innerMesh = new THREE.Mesh(innerGeo, innerMat);
                group.add(innerMesh);
                
                let qty = params.ventConfig.quantity;
                let spacing = params.ventConfig.spacing * scale;
                let dia = params.ventConfig.holeDiameter * scale;
                
                if (qty > 0) {{
                    let ventMat = new THREE.MeshBasicMaterial({{ color: 0x020617 }});
                    let span = qty * spacing;
                    let startX = -span / 2 + spacing / 2;
                    
                    for (let i = 0; i < qty; i++) {{
                        let xPos = startX + i * spacing;
                        let ventGeo = new THREE.CylinderGeometry(dia / 2, dia / 2, t * 2.2, 16);
                        let ventMesh = new THREE.Mesh(ventGeo, ventMat);
                        ventMesh.position.set(xPos, h / 2, 0);
                        group.add(ventMesh);
                    }}
                }}
            }};
            
            if (!window.THREE) {{
                let script = document.createElement('script');
                script.src = 'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js';
                script.onload = () => {{ init(); }};
                document.head.appendChild(script);
            }} else {{
                init();
            }}
            "#,
            serde_json::to_string(&p).unwrap()
        );
        let _ = eval(&js);
    });

    use_effect(move || {
        spawn(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.get("http://localhost:3000/api/parameters").send().await {
                if resp.status().is_success() {
                    if let Ok(data) = resp.json::<EnclosureParams>().await {
                        params.set(data);
                    }
                }
            }
        });
    });

    let sync_to_backend = move |new_p: EnclosureParams| {
        spawn(async move {
            let client = reqwest::Client::new();
            let _ = client.post("http://localhost:3000/api/parameters")
                .json(&new_p)
                .send()
                .await;
        });
    };

    let p = params.read().clone();

    rsx! {
        div {
            class: "flex h-full flex-1 bg-slate-900 overflow-hidden text-xs",
            
            // Sidebar Controls
            div {
                class: "w-80 bg-slate-950 border-r border-slate-800 p-6 flex flex-col gap-6 overflow-y-auto",
                div {
                    class: "border-b border-slate-800 pb-3",
                    h2 { class: "text-base font-bold text-slate-100", "Enclosure Designer" }
                    p { class: "text-[10px] text-slate-500", "Configure 3D CAD dimensions in real-time" }
                }

                // Dimensions Group
                div {
                    class: "flex flex-col gap-3",
                    h3 { class: "font-semibold text-slate-400 uppercase text-[9px]", "Dimensions" }
                    
                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Width" } span { "{p.dimensions.width} mm" } }
                        input {
                            type: "range", min: "50", max: "200", value: "{p.dimensions.width}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.dimensions.width = e.value().parse().unwrap_or(current.dimensions.width);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }

                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Height" } span { "{p.dimensions.height} mm" } }
                        input {
                            type: "range", min: "40", max: "150", value: "{p.dimensions.height}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.dimensions.height = e.value().parse().unwrap_or(current.dimensions.height);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }

                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Depth" } span { "{p.dimensions.depth} mm" } }
                        input {
                            type: "range", min: "30", max: "120", value: "{p.dimensions.depth}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.dimensions.depth = e.value().parse().unwrap_or(current.dimensions.depth);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }
                }

                // Wall & Material Group
                div {
                    class: "flex flex-col gap-4",
                    h3 { class: "font-semibold text-slate-400 uppercase text-[9px]", "Material & Shell" }
                    
                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Wall Thickness" } span { "{p.wall_thickness} mm" } }
                        input {
                            type: "range", min: "1", max: "10", step: "0.5", value: "{p.wall_thickness}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.wall_thickness = e.value().parse().unwrap_or(current.wall_thickness);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }

                    div {
                        class: "flex flex-col gap-2",
                        span { class: "text-slate-300", "Material Finish" }
                        div {
                            class: "grid grid-cols-3 gap-2",
                            for mat in &["pla", "abs", "aluminum"] {
                                button {
                                    class: format!("py-2 rounded-lg font-semibold transition-all {}", if p.material == *mat { "bg-sky-600 text-white shadow-lg shadow-sky-600/30" } else { "bg-slate-800 text-slate-400 hover:bg-slate-700" }),
                                    onclick: move |_| {
                                        let mut current = params.read().clone();
                                        current.material = mat.to_string();
                                        params.set(current.clone());
                                        sync_to_backend(current);
                                    },
                                    "{mat.to_uppercase()}"
                                }
                            }
                        }
                    }
                }

                // Vents Configuration Group
                div {
                    class: "flex flex-col gap-3",
                    h3 { class: "font-semibold text-slate-400 uppercase text-[9px]", "Ventilation Holes" }
                    
                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Hole Diameter" } span { "{p.vent_config.hole_diameter} mm" } }
                        input {
                            type: "range", min: "1", max: "8", step: "0.5", value: "{p.vent_config.hole_diameter}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.vent_config.hole_diameter = e.value().parse().unwrap_or(current.vent_config.hole_diameter);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }

                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Spacing" } span { "{p.vent_config.spacing} mm" } }
                        input {
                            type: "range", min: "2", max: "15", value: "{p.vent_config.spacing}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.vent_config.spacing = e.value().parse().unwrap_or(current.vent_config.spacing);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }

                    div {
                        class: "flex flex-col gap-1",
                        div { class: "flex justify-between text-slate-300", span { "Quantity" } span { "{p.vent_config.quantity}" } }
                        input {
                            type: "range", min: "0", max: "30", value: "{p.vent_config.quantity}",
                            class: "w-full accent-sky-500 h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer",
                            oninput: move |e| {
                                let mut current = params.read().clone();
                                current.vent_config.quantity = e.value().parse().unwrap_or(current.vent_config.quantity);
                                params.set(current.clone());
                                sync_to_backend(current);
                            }
                        }
                    }
                }
            }
            
            // 3D Canvas
            div {
                class: "flex-1 h-full relative",
                canvas {
                    id: "threejs-canvas",
                    class: "w-full h-full block"
                }
            }
        }
    }
}
