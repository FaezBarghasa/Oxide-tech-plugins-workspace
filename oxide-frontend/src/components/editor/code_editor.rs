use dioxus::prelude::*;
use dioxus::document::eval;
use crate::state::*;

#[component]
pub fn CodeEditor(mut editor_state: Signal<EditorState>) -> Element {
    let content = editor_state.read().content.clone();
    
    use_effect(move || {
        let current_content = editor_state.read().content.clone();
        let escaped_content = current_content
            .replace('\\', "\\\\")
            .replace('`', "\\`")
            .replace('$', "\\$");
            
        let js = format!(
            r#"
            let content = `{}`;
            let init = () => {{
                let container = document.getElementById('monaco-editor-container');
                if (!container) return;
                
                if (window.monacoEditorInstance) {{
                    if (window.monacoEditorInstance.getValue() !== content) {{
                        window.monacoEditorInstance.setValue(content);
                    }}
                    return;
                }}
                
                require.config({{ paths: {{ vs: 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.39.0/min/vs' }} }});
                require(['vs/editor/editor.main'], function() {{
                    let editor = monaco.editor.create(container, {{
                        value: content,
                        language: 'rust',
                        theme: 'vs-dark',
                        automaticLayout: true,
                        fontSize: 14,
                        fontFamily: 'Fira Code, monospace',
                        minimap: {{ enabled: false }},
                    }});
                    window.monacoEditorInstance = editor;
                    
                    editor.onDidChangeModelContent(() => {{
                        dioxus.send(editor.getValue());
                    }});
                }});
            }};
            
            if (!window.require) {{
                let script = document.createElement('script');
                script.src = 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.39.0/min/vs/loader.min.js';
                script.onload = init;
                document.head.appendChild(script);
            }} else {{
                init();
            }}
            "#,
            escaped_content
        );

        let mut monaco_eval = eval(&js);
        
        spawn(async move {
            while let Ok(val) = monaco_eval.recv::<String>().await {
                editor_state.write().content = val;
            }
        });
    });

    rsx! {
        div {
            class: "flex flex-col flex-1 bg-slate-900 border-r border-slate-700",
            div {
                class: "flex items-center justify-between px-4 py-2 bg-slate-800 border-b border-slate-700",
                div {
                    class: "flex items-center gap-2",
                    span { class: "w-2.5 h-2.5 rounded-full bg-emerald-500" }
                    span { class: "text-xs font-medium text-gray-300", "main.rs" }
                }
                span { class: "text-[11px] text-gray-500", "Rust Coder IDE" }
            }
            div {
                class: "flex-1 w-full h-full min-h-[300px]",
                id: "monaco-editor-container",
            }
        }
    }
}
