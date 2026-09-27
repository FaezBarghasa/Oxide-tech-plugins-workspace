#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::models::{
    AutomationSkill, McpExtension, JetBrainsExtension, JetbrainsState, Message, MessageRole,
    PRReviewIssue, PRReviewResult, ChatRequest, ChatResponse, PRReviewRequest, SimulateResponse,
    JetbrainsApiResponse, SkillApiResponse, AddJetBrainsExtensionPayload, AddMcpExtensionPayload,
    AddSkillPayload, RunSkillPayload, SimulatePayload, TogglePayload,
};

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    Explorer,
    Skills,
    Mcp,
    Review,
    Jetbrains,
}

pub fn App(cx: Scope) -> Element {
    let messages = use_state(cx, || vec![
        Message {
            role: MessageRole::User,
            content: "Hello! I am the Oxide-Tech Local Agent OS Model. Ask me to generate embedded Rust HAL wrappers, design Embassy async executors, specify RTIC resources, or resolve datasheet hardware constraints! How can I assist you with embedded firmware development today?".to_string(),
        },
    ]);
    let input_val = use_state(cx, String::new);
    let is_loading = use_state(cx, || false);
    let chat_mode = use_state(cx, || "executing".to_string());
    let active_tab = use_state(cx, || ActiveTab::Skills);

    let skills = use_state(cx, Vec::<AutomationSkill>::new);
    let selected_skill = use_state(cx, || None::<AutomationSkill>);
    let is_registering = use_state(cx, || false);

    let new_skill_name = use_state(cx, String::new);
    let new_skill_desc = use_state(cx, String::new);
    let new_skill_script = use_state(cx, String::new);
    let new_skill_type = use_state(cx, || "bash".to_string());
    let new_skill_trigger = use_state(cx, || "Manual".to_string());

    let mcp_servers = use_state(cx, Vec::<McpExtension>::new);
    let is_adding_mcp = use_state(cx, || false);

    let new_mcp_name = use_state(cx, String::new);
    let new_mcp_desc = use_state(cx, String::new);
    let new_mcp_url = use_state(cx, String::new);
    let new_mcp_caps = use_state(cx, || "tools/read_hardware, tools/datasheet_lookup".to_string());

    let review_title = use_state(cx, || "Fix circular telemetry buffer".to_string());
    let review_desc = use_state(cx, || "Introduces direct heap FIFO queue metrics.".to_string());
    let review_code = use_state(cx, || "#![no_std]\n// Intended as premium ring telemetry driver\nuse std::collections::VecDeque;\n\npub struct TelemetryBuffer {\n    data: VecDeque<u32>,\n}\n\nimpl TelemetryBuffer {\n    pub fn new() -> Self {\n        Self { data: VecDeque::new() }\n    }\n}".to_string());
    let is_submitting_review = use_state(cx, || false);
    let review_result = use_state(cx, || None::<PRReviewResult>);

    let jb_state = use_state(cx, || JetbrainsState {
        connected: true,
        port: 8085,
        ide_version: "2024.2 (IntelliJ IDEA Community)".to_string(),
        plugin_build: "v0.1.0-alpha.4".to_string(),
        last_handshake: "".to_string(),
        active_language: "Rust / Kotlin".to_string(),
        gradle_task_executing: false,
        gradle_log: "".to_string(),
    });
    let jb_extensions = use_state(cx, Vec::<JetBrainsExtension>::new);
    let is_adding_jb_ext = use_state(cx, || false);

    let new_jb_name = use_state(cx, String::new);
    let new_jb_type = use_state(cx, || "Annotator".to_string());
    let new_jb_class = use_state(cx, String::new);
    let new_jb_desc = use_state(cx, String::new);

    let sim_event_type = use_state(cx, || "completion".to_string());
    let sim_value = use_state(cx, || "TIM2".to_string());
    let sim_log = use_state(cx, String::new);
    let sim_result = use_state(cx, || None::<serde_json::Value>);

    let client = reqwest::Client::new();

    let fetch_data = move |_| {
        cx.spawn({
            let skills = skills.clone();
            let selected_skill = selected_skill.clone();
            let mcp_servers = mcp_servers.clone();
            let jb_state = jb_state.clone();
            let jb_extensions = jb_extensions.clone();
            let client = client.clone();
            async move {
                if let Ok(data) = client.get("/api/skills").send().await.unwrap().json::<Vec<AutomationSkill>>().await {
                    if data.len() > 0 && selected_skill.is_none() {
                        selected_skill.set(Some(data[0].clone()));
                    }
                    skills.set(data);
                }
                if let Ok(data) = client.get("/api/mcp").send().await.unwrap().json::<Vec<McpExtension>>().await {
                    mcp_servers.set(data);
                }
                if let Ok(data) = client.get("/api/jetbrains").send().await.unwrap().json::<JetbrainsApiResponse>().await {
                    jb_state.set(data.state);
                    jb_extensions.set(data.extensions);
                }
            }
        });
    };

    use_effect(cx, (), fetch_data);

    cx.render(rsx! {
        div {
            class: "min-h-screen bg-[#07090e] text-slate-200 font-sans p-3 md:p-5 flex flex-col items-center",
            div {
                class: "max-w-6xl w-full flex flex-col min-h-[calc(100vh-2rem)]",
                Header {}
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-4 flex-1 items-stretch",
                    LeftPanel {
                        active_tab: active_tab.clone(),
                        skills: skills.clone(),
                        selected_skill: selected_skill.clone(),
                        is_registering: is_registering.clone(),
                        new_skill_name: new_skill_name.clone(),
                        new_skill_desc: new_skill_desc.clone(),
                        new_skill_script: new_skill_script.clone(),
                        new_skill_type: new_skill_type.clone(),
                        new_skill_trigger: new_skill_trigger.clone(),
                        mcp_servers: mcp_servers.clone(),
                        is_adding_mcp: is_adding_mcp.clone(),
                        new_mcp_name: new_mcp_name.clone(),
                        new_mcp_desc: new_mcp_desc.clone(),
                        new_mcp_url: new_mcp_url.clone(),
                        new_mcp_caps: new_mcp_caps.clone(),
                        review_title: review_title.clone(),
                        review_desc: review_desc.clone(),
                        review_code: review_code.clone(),
                        is_submitting_review: is_submitting_review.clone(),
                        review_result: review_result.clone(),
                        jb_state: jb_state.clone(),
                        jb_extensions: jb_extensions.clone(),
                        is_adding_jb_ext: is_adding_jb_ext.clone(),
                        new_jb_name: new_jb_name.clone(),
                        new_jb_type: new_jb_type.clone(),
                        new_jb_class: new_jb_class.clone(),
                        new_jb_desc: new_jb_desc.clone(),
                        sim_event_type: sim_event_type.clone(),
                        sim_value: sim_value.clone(),
                        sim_log: sim_log.clone(),
                        sim_result: sim_result.clone(),
                    }
                    ChatPanel {
                        messages: messages.clone(),
                        input_val: input_val.clone(),
                        is_loading: is_loading.clone(),
                        chat_mode: chat_mode.clone(),
                    }
                }
            }
        }
    })
}

#[inline_props]
fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header {
            class: "mb-4 flex flex-col sm:flex-row justify-between items-start sm:items-center border-b border-slate-800/60 pb-3 gap-2",
            div {
                div {
                    class: "flex flex-wrap items-center gap-2 mb-1",
                    span {
                        class: "inline-flex items-center px-1.5 py-0.2 rounded text-[10px] font-mono bg-cyan-950/50 text-cyan-400 border border-cyan-800/30",
                        "Agent Active (Live 3000)"
                    }
                    span {
                        class: "inline-flex items-center px-1.5 py-0.2 rounded text-[10px] font-mono bg-orange-950/40 text-orange-400 border border-orange-950/50",
                        "STM32H7 / Cortex-M7"
                    }
                }
                h1 {
                    class: "text-xl font-semibold tracking-tight text-white flex items-center gap-1.5",
                    "Oxide-Tech Console"
                }
            }
            p {
                class: "text-slate-400 text-xs max-w-sm self-end sm:text-right hidden sm:block leading-normal",
                "Gradle automation & script telemetry pipeline."
            }
        }
    })
}

#[inline_props]
fn LeftPanel(cx: Scope,
    active_tab: UseState<ActiveTab>,
    skills: UseState<Vec<AutomationSkill>>,
    selected_skill: UseState<Option<AutomationSkill>>,
    is_registering: UseState<bool>,
    new_skill_name: UseState<String>,
    new_skill_desc: UseState<String>,
    new_skill_script: UseState<String>,
    new_skill_type: UseState<String>,
    new_skill_trigger: UseState<String>,
    mcp_servers: UseState<Vec<McpExtension>>,
    is_adding_mcp: UseState<bool>,
    new_mcp_name: UseState<String>,
    new_mcp_desc: UseState<String>,
    new_mcp_url: UseState<String>,
    new_mcp_caps: UseState<String>,
    review_title: UseState<String>,
    review_desc: UseState<String>,
    review_code: UseState<String>,
    is_submitting_review: UseState<bool>,
    review_result: UseState<Option<PRReviewResult>>,
    jb_state: UseState<JetbrainsState>,
    jb_extensions: UseState<Vec<JetBrainsExtension>>,
    is_adding_jb_ext: UseState<bool>,
    new_jb_name: UseState<String>,
    new_jb_type: UseState<String>,
    new_jb_class: UseState<String>,
    new_jb_desc: UseState<String>,
    sim_event_type: UseState<String>,
    sim_value: UseState<String>,
    sim_log: UseState<String>,
    sim_result: UseState<Option<serde_json::Value>>,
) -> Element {
    cx.render(rsx! {
        div {
            class: "lg:col-span-5 flex flex-col gap-3",
            div {
                class: "flex bg-[#111520] border border-slate-800 p-0.5 rounded-xl gap-0.5",
                TabButton {
                    label: "Skills",
                    is_active: **active_tab == ActiveTab::Skills,
                    onclick: move |_| active_tab.set(ActiveTab::Skills),
                }
                TabButton {
                    label: "MCP",
                    is_active: **active_tab == ActiveTab::Mcp,
                    onclick: move |_| active_tab.set(ActiveTab::Mcp),
                }
                TabButton {
                    label: "JetBrains",
                    is_active: **active_tab == ActiveTab::Jetbrains,
                    onclick: move |_| active_tab.set(ActiveTab::Jetbrains),
                }
                TabButton {
                    label: "PR Review",
                    is_active: **active_tab == ActiveTab::Review,
                    onclick: move |_| active_tab.set(ActiveTab::Review),
                }
                TabButton {
                    label: "Files",
                    is_active: **active_tab == ActiveTab::Explorer,
                    onclick: move |_| active_tab.set(ActiveTab::Explorer),
                }
            }
            match **active_tab {
                ActiveTab::Skills => rsx!{ SkillsPanel {
                    skills: skills.clone(),
                    selected_skill: selected_skill.clone(),
                    is_registering: is_registering.clone(),
                    new_skill_name: new_skill_name.clone(),
                    new_skill_desc: new_skill_desc.clone(),
                    new_skill_script: new_skill_script.clone(),
                    new_skill_type: new_skill_type.clone(),
                    new_skill_trigger: new_skill_trigger.clone(),
                }},
                ActiveTab::Mcp => rsx!{ McpPanel {
                    mcp_servers: mcp_servers.clone(),
                    is_adding_mcp: is_adding_mcp.clone(),
                    new_mcp_name: new_mcp_name.clone(),
                    new_mcp_desc: new_mcp_desc.clone(),
                    new_mcp_url: new_mcp_url.clone(),
                    new_mcp_caps: new_mcp_caps.clone(),
                }},
                ActiveTab::Jetbrains => rsx!{ JetbrainsPanel {
                    jb_state: jb_state.clone(),
                    jb_extensions: jb_extensions.clone(),
                    is_adding_jb_ext: is_adding_jb_ext.clone(),
                    new_jb_name: new_jb_name.clone(),
                    new_jb_type: new_jb_type.clone(),
                    new_jb_class: new_jb_class.clone(),
                    new_jb_desc: new_jb_desc.clone(),
                    sim_event_type: sim_event_type.clone(),
                    sim_value: sim_value.clone(),
                    sim_log: sim_log.clone(),
                    sim_result: sim_result.clone(),
                }},
                ActiveTab::Review => rsx!{ ReviewPanel {
                    review_title: review_title.clone(),
                    review_desc: review_desc.clone(),
                    review_code: review_code.clone(),
                    is_submitting_review: is_submitting_review.clone(),
                    review_result: review_result.clone(),
                }},
                ActiveTab::Explorer => rsx!{ ExplorerPanel {} },
            }
        }
    })
}

#[inline_props]
fn TabButton<'a>(cx: Scope<'a>, label: &'a str, is_active: bool, onclick: EventHandler<'a, MouseEvent>) -> Element<'a> {
    let active_class = if *is_active { "bg-cyan-950/60 text-cyan-400 border border-cyan-800/10" } else { "text-slate-400 hover:text-white" };
    cx.render(rsx! {
        button {
            class: "flex-1 py-1 px-1 rounded-lg text-[9px] font-medium font-mono flex flex-col sm:flex-row items-center justify-center gap-1 transition-all {active_class}",
            onclick: move |evt| onclick.call(evt),
            span { "{label}" }
        }
    })
}

// ... Implement SkillsPanel, McpPanel, JetbrainsPanel, ReviewPanel, ExplorerPanel, and ChatPanel here
// This is a simplified version of the panels to keep the code concise.
// A full implementation would require creating each of these components with their respective UI and logic.

#[inline_props]
fn SkillsPanel(cx: Scope,
    skills: UseState<Vec<AutomationSkill>>,
    selected_skill: UseState<Option<AutomationSkill>>,
    is_registering: UseState<bool>,
    new_skill_name: UseState<String>,
    new_skill_desc: UseState<String>,
    new_skill_script: UseState<String>,
    new_skill_type: UseState<String>,
    new_skill_trigger: UseState<String>,
) -> Element {
    cx.render(rsx!{
        div {
            class: "bg-[#111520] border border-slate-800 rounded-xl overflow-hidden flex flex-col flex-1 p-3.5 gap-3",
            "Skills Panel"
        }
    })
}

#[inline_props]
fn McpPanel(cx: Scope,
    mcp_servers: UseState<Vec<McpExtension>>,
    is_adding_mcp: UseState<bool>,
    new_mcp_name: UseState<String>,
    new_mcp_desc: UseState<String>,
    new_mcp_url: UseState<String>,
    new_mcp_caps: UseState<String>,
) -> Element {
    cx.render(rsx!{
        div {
            class: "bg-[#111520] border border-slate-800 rounded-xl overflow-hidden flex flex-col flex-1 p-3.5 gap-3",
            "MCP Panel"
        }
    })
}

#[inline_props]
fn JetbrainsPanel(cx: Scope,
    jb_state: UseState<JetbrainsState>,
    jb_extensions: UseState<Vec<JetBrainsExtension>>,
    is_adding_jb_ext: UseState<bool>,
    new_jb_name: UseState<String>,
    new_jb_type: UseState<String>,
    new_jb_class: UseState<String>,
    new_jb_desc: UseState<String>,
    sim_event_type: UseState<String>,
    sim_value: UseState<String>,
    sim_log: UseState<String>,
    sim_result: UseState<Option<serde_json::Value>>,
) -> Element {
    cx.render(rsx!{
        div {
            class: "bg-[#111520] border border-slate-800 rounded-xl overflow-hidden flex flex-col flex-1 p-3.5 gap-3",
            "Jetbrains Panel"
        }
    })
}

#[inline_props]
fn ReviewPanel(cx: Scope,
    review_title: UseState<String>,
    review_desc: UseState<String>,
    review_code: UseState<String>,
    is_submitting_review: UseState<bool>,
    review_result: UseState<Option<PRReviewResult>>,
) -> Element {
    cx.render(rsx!{
        div {
            class: "bg-[#111520] border border-slate-800 rounded-xl overflow-hidden flex flex-col flex-1 p-3.5 gap-3",
            "Review Panel"
        }
    })
}

#[inline_props]
fn ExplorerPanel(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            class: "bg-[#111520] border border-slate-800 rounded-xl overflow-hidden shadow-xl flex flex-col flex-1",
            "Explorer Panel"
        }
    })
}

#[inline_props]
fn ChatPanel(cx: Scope,
    messages: UseState<Vec<Message>>,
    input_val: UseState<String>,
    is_loading: UseState<bool>,
    chat_mode: UseState<String>,
) -> Element {
    cx.render(rsx!{
        div {
            class: "lg:col-span-7 flex flex-col bg-[#111520] border border-slate-800 rounded-xl overflow-hidden shadow-2xl min-h-[400px]",
            "Chat Panel"
        }
    })
}
