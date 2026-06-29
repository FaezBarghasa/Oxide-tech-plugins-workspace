#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use dioxus::document::eval;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::CADState;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn Scene() -> Element {
    let state: CADState = use_context::<CADState>();
    
    use_effect(move || {
        let params_val = state.params.read().clone();
        let js_code = format!(
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
                scene.background = new THREE.Color(0x18181b);
                window.blenderScene = scene;
                
                let camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
                camera.position.set(12, 12, 12);
                camera.lookAt(0, 0, 0);
                window.blenderCamera = camera;
                
                let renderer = new THREE.WebGLRenderer({{ canvas: canvas, antialias: true }});
                renderer.setSize(width, height);
                renderer.shadowMap.enabled = true;
                window.blenderRenderer = renderer;
                
                let gridHelper = new THREE.GridHelper(20, 20, 0x27272a, 0x09090b);
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
                    group.rotation.y += 0.005;
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
                if (params.material === 'abs') matColor = 0xaaaaaa;
                if (params.material === 'petg') matColor = 0xdddddd;
                if (params.material === 'aluminum') {{
                    matColor = 0x999999;
                    metalness = 0.8;
                }}
                
                let material = new THREE.MeshStandardMaterial({{
                    color: matColor,
                    roughness: 0.5,
                    metalness: metalness,
                    transparent: true,
                    opacity: 0.8
                }});
                
                let outerGeo = new THREE.BoxGeometry(w, h, d);
                let outerMesh = new THREE.Mesh(outerGeo, material);
                group.add(outerMesh);
                
                let innerGeo = new THREE.BoxGeometry(w - t*2, h - t*2, d - t*2);
                let innerMat = new THREE.MeshBasicMaterial({{ color: 0x0f172a, wireframe: true, opacity: 0.2, transparent: true }});
                let innerMesh = new THREE.Mesh(innerGeo, innerMat);
                group.add(innerMesh);
                
                let qty = params.ventConfig.quantity;
                let spacing = params.ventConfig.spacing * scale;
                let dia = params.ventConfig.holeDiameter * scale;
                
                if (qty > 0) {{
                    let ventMat = new THREE.MeshBasicMaterial({{ color: 0x1e293b }});
                    let span = qty * spacing;
                    let startX = -span / 2 + spacing / 2;
                    
                    for (let i = 0; i < qty; i++) {{
                        let xPos = startX + i * spacing;
                        let ventGeo = new THREE.CylinderGeometry(dia / 2, dia / 2, t * 2, 16);
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
            serde_json::to_string(&params_val).unwrap()
        );
        let _ = eval(&js_code);
    });

    rsx! {
        div {
            style: "width: 100%; height: 100%; position: relative; background-color: #18181b;",
            canvas {
                id: "threejs-canvas",
                style: "width: 100%; height: 100%; display: block;"
            }
        }
    }
}
