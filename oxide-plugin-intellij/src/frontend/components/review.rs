#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::{IntellijState, PRReviewResult};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ReviewPanel() -> Element {
    let mut state: IntellijState = use_context::<IntellijState>();

    let mut title = state.review_title;
    let mut desc = state.review_desc;
    let mut code = state.review_code;
    let result = state.review_result.read().clone();
    let submitting = *state.review_submitting.read();

    let trigger_review = move |_| {
        if submitting {
            return;
        }
        state.review_submitting.set(true);

        let t = title.read().clone();
        let d = desc.read().clone();
        let c = code.read().clone();

        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "title": t,
                "description": d,
                "code": c,
            });

            if let Ok(resp) = client.post("/api/review").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(report) = resp.json::<PRReviewResult>().await {
                        state.review_result.set(Some(report));
                    }
                }
            }
            state.review_submitting.set(false);
        });
    };

    rsx! {
        div {
            style: "background: rgba(17, 21, 32, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; flex-grow: 1; padding: 16px; gap: 16px; max-height: calc(100vh - 120px); overflow-y: auto;",
            
            h2 {
                style: "font-size: 14px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: #f1f5f9; margin: 0; text-align: left;",
                "Static Audit Plan"
            }

            // Input Fields
            div {
                style: "display: flex; flex-direction: column; gap: 10px; background: rgba(15, 19, 34, 0.3); border: 1px solid rgba(255, 255, 255, 0.03); border-radius: 10px; padding: 12px;",
                
                div {
                    style: "display: flex; gap: 10px;",
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 9px; color: #64748b; font-family: monospace; text-align: left;", "PR Title" }
                        input {
                            style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff;",
                            value: "{title}",
                            oninput: move |e| title.set(e.value().clone()),
                        }
                    }
                    div {
                        style: "flex: 2; display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 9px; color: #64748b; font-family: monospace; text-align: left;", "PR Description" }
                        input {
                            style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff;",
                            value: "{desc}",
                            oninput: move |e| desc.set(e.value().clone()),
                        }
                    }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 4px; margin-top: 4px;",
                    label { style: "font-size: 9px; color: #64748b; font-family: monospace; text-align: left;", "Code Context Editor" }
                    textarea {
                        style: "background: #07090f; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 10px; font-family: monospace; font-size: 10px; color: #ffffff; min-height: 120px; resize: vertical; line-height: 1.4;",
                        value: "{code}",
                        oninput: move |e| code.set(e.value().clone()),
                    }
                }

                button {
                    style: "background: #0891b2; color: #ffffff; border: none; padding: 10px; border-radius: 8px; font-size: 11px; font-weight: 700; cursor: pointer; transition: all 0.2s; margin-top: 8px;",
                    disabled: submitting,
                    onclick: trigger_review,
                    if submitting { "Auditing Code Context..." } else { "Submit for Co-Pilot Audit" }
                }
            }

            // Results Section
            {if let Some(res) = result {
                let status_text = if res.approved { "PASSED" } else { "FAILED" };
                let status_bg = if res.approved { "rgba(34, 197, 94, 0.1)" } else { "rgba(239, 68, 68, 0.1)" };
                let status_border = if res.approved { "1px solid rgba(34, 197, 94, 0.3)" } else { "1px solid rgba(239, 68, 68, 0.3)" };
                let status_color = if res.approved { "#4ade80" } else { "#f87171" };

                rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px; animation: fadeIn 0.3s;",
                        
                        // Scorecard
                        div {
                            style: "background: {status_bg}; border: {status_border}; border-radius: 10px; padding: 12px; display: flex; align-items: center; justify-content: space-between;",
                            div {
                                style: "text-align: left;",
                                div { style: "font-size: 9px; font-family: monospace; color: #94a3b8; font-weight: 600;", "AUDIT STATUS" }
                                div { style: "font-size: 16px; font-weight: 800; color: {status_color}; margin-top: 2px;", "{status_text}" }
                            }
                            div {
                                style: "text-align: right;",
                                div { style: "font-size: 9px; font-family: monospace; color: #94a3b8; font-weight: 600;", "COMPLIANCE SCORE" }
                                div { style: "font-size: 18px; font-weight: 800; color: #ffffff; margin-top: 2px;", "{res.score} / 100" }
                            }
                        }

                        // Summary
                        div {
                            style: "background: rgba(255, 255, 255, 0.02); border: 1px solid rgba(255, 255, 255, 0.04); border-radius: 8px; padding: 10px; text-align: left;",
                            h4 { style: "font-size: 10px; font-weight: 600; color: #cbd5e1; margin: 0 0 4px 0;", "Audit Summary" }
                            p { style: "font-size: 10px; color: #94a3b8; margin: 0; line-height: 1.4;", "{res.summary}" }
                        }

                        // Issues table
                        if !res.issues.is_empty() {
                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",
                                h3 { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; color: #94a3b8; margin: 0; text-align: left;", "Violations Detected ({res.issues.len()})" }
                                
                                div {
                                    style: "display: flex; flex-direction: column; gap: 6px;",
                                    {res.issues.iter().map(|issue| {
                                        let sev_color = match issue.severity.as_str() {
                                            "Critical" => "#ef4444",
                                            "Warning" => "#f59e0b",
                                            _ => "#06b6d4"
                                        };
                                        let sev_bg = match issue.severity.as_str() {
                                            "Critical" => "rgba(239, 68, 68, 0.08)",
                                            "Warning" => "rgba(245, 158, 11, 0.08)",
                                            _ => "rgba(6, 182, 212, 0.08)"
                                        };

                                        rsx! {
                                            div {
                                                key: "{issue.title}",
                                                style: "background: {sev_bg}; border: 1px solid rgba(255, 255, 255, 0.03); border-radius: 8px; padding: 10px; text-align: left; display: flex; flex-direction: column; gap: 4px;",
                                                
                                                div {
                                                    style: "display: flex; justify-content: space-between; align-items: center;",
                                                    span {
                                                        style: "font-size: 11px; font-weight: 700; color: #ffffff;",
                                                        "{issue.title}"
                                                    }
                                                    span {
                                                        style: "font-family: monospace; font-size: 8px; padding: 2px 6px; border-radius: 4px; background: rgba(255, 255, 255, 0.04); color: {sev_color}; border: 1px solid {sev_color}; font-weight: bold;",
                                                        "{issue.severity.to_uppercase()}"
                                                    }
                                                }

                                                div {
                                                    style: "font-size: 10px; color: #cbd5e1; margin-top: 2px;",
                                                    "{issue.explanation}"
                                                }

                                                div {
                                                    style: "font-size: 10px; color: #34d399; margin-top: 4px; font-family: monospace; border-top: 1px dashed rgba(255, 255, 255, 0.06); padding-top: 4px; display: flex; gap: 4px;",
                                                    span { style: "color: #64748b;", "Remedy:" }
                                                    span { "{issue.recommendation}" }
                                                }

                                                {issue.line.map(|line| rsx! {
                                                    div {
                                                        style: "font-family: monospace; font-size: 9px; color: #64748b; margin-top: 2px;",
                                                        "Location: Line {line}"
                                                    }
                                                })}
                                            }
                                        }
                                    })}
                                }
                            }
                        } else {
                            div { style: "color: #4ade80; font-size: 11px; font-family: monospace; text-align: center; padding: 10px;", "✔ Code complies cleanly with all bare-metal limits!" }
                        }
                    }
                }
            } else {
                rsx! {
                    div { style: "color: #64748b; font-size: 11px; text-align: center; padding: 20px;", "Submit a PR code snippet above to generate compliance review scorecards." }
                }
            }}
        }
    }
}
