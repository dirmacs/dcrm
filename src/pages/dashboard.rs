use crate::models::{Activity, ActivityType, DealStage};
use crate::state::{Modal, use_app_data, use_modal};
use dioxus::prelude::*;

#[component]
pub fn DashboardPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();

    let total_contacts = data.read().contacts.len();
    let active_deals = data.read().active_deals_count();
    let pipeline_value = data.read().total_pipeline_value();
    let won_value = data.read().won_deals_value();
    let pending_tasks = data.read().pending_tasks_count();
    let recent_activities: Vec<Activity> = data
        .read()
        .recent_activities(5)
        .into_iter()
        .cloned()
        .collect();

    rsx! {
        div { class: "flex-1 overflow-y-auto p-6",
            // Stats Grid
            div { class: "grid grid-cols-4 gap-4 mb-6",
                StatCard {
                    label: "Total Contacts",
                    value: total_contacts.to_string(),
                    icon: "◎",
                    color: "text-zinc-100",
                }
                StatCard {
                    label: "Active Deals",
                    value: active_deals.to_string(),
                    icon: "◈",
                    color: "text-zinc-100",
                }
                StatCard {
                    label: "Pipeline Value",
                    value: format_currency(pipeline_value),
                    icon: "◆",
                    color: "text-accent",
                }
                StatCard {
                    label: "Won Revenue",
                    value: format_currency(won_value),
                    icon: "✓",
                    color: "text-emerald-400",
                }
            }

            // Two-column layout
            div { class: "grid grid-cols-2 gap-6",
                // Pipeline Overview
                div { class: "bg-dark-800 border border-zinc-800 rounded-xl overflow-hidden",
                    div { class: "flex items-center justify-between px-5 py-4 border-b border-zinc-800",
                        h3 { class: "text-sm font-semibold text-zinc-100", "Pipeline Overview" }
                    }
                    div { class: "p-5",
                        for stage in DealStage::active() {
                            PipelineStageRow {
                                stage: stage,
                                count: data.read().deals_by_stage(stage).len(),
                                value: data.read().deals_by_stage(stage).iter().map(|d| d.value).sum::<f64>(),
                            }
                        }
                    }
                }

                // Recent Activity
                div { class: "bg-dark-800 border border-zinc-800 rounded-xl overflow-hidden",
                    div { class: "flex items-center justify-between px-5 py-4 border-b border-zinc-800",
                        h3 { class: "text-sm font-semibold text-zinc-100", "Recent Activity" }
                        button {
                            class: "text-sm text-zinc-400 hover:text-zinc-100 transition-colors",
                            "View All"
                        }
                    }
                    div { class: "p-5",
                        if recent_activities.is_empty() {
                            div { class: "flex flex-col items-center justify-center py-8 text-center",
                                div { class: "text-zinc-100 font-medium", "No activities yet" }
                                div { class: "text-sm text-zinc-500", "Start logging your interactions" }
                            }
                        } else {
                            div { class: "space-y-1",
                                for activity in recent_activities {
                                    ActivityRow {
                                        icon: activity.activity_type.icon().to_string(),
                                        title: activity.title.clone(),
                                        meta: activity.format_date(),
                                        completed: activity.completed,
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Pending Tasks
            div { class: "bg-dark-800 border border-zinc-800 rounded-xl overflow-hidden mt-6",
                div { class: "flex items-center justify-between px-5 py-4 border-b border-zinc-800",
                    div { class: "flex items-center gap-2",
                        h3 { class: "text-sm font-semibold text-zinc-100", "Pending Tasks" }
                        if pending_tasks > 0 {
                            span {
                                class: "text-xs bg-amber-500/15 text-amber-400 px-2 py-0.5 rounded-full",
                                "{pending_tasks}"
                            }
                        }
                    }
                    button {
                        class: "px-3 py-1.5 text-sm bg-dark-700 border border-zinc-700 text-zinc-100 rounded-md hover:bg-zinc-700 transition-colors",
                        onclick: move |_| modal.set(Modal::NewActivity),
                        "+ Add Task"
                    }
                }
                div { class: "p-5",
                    {
                        let tasks: Vec<_> = data.read().activities.iter()
                            .filter(|a| a.activity_type == ActivityType::Task && !a.completed)
                            .cloned()
                            .collect();

                        if tasks.is_empty() {
                            rsx! {
                                div { class: "flex flex-col items-center justify-center py-8 text-center",
                                    div { class: "w-12 h-12 bg-dark-700 rounded-full flex items-center justify-center text-xl text-zinc-500 mb-3",
                                        "✓"
                                    }
                                    div { class: "text-zinc-100 font-medium", "All caught up!" }
                                    div { class: "text-sm text-zinc-500", "No pending tasks" }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "space-y-1",
                                    for task in tasks.iter().take(5) {
                                        TaskRow {
                                            id: task.id.clone(),
                                            title: task.title.clone(),
                                            due: task.due_date.map(|d| d.format("%b %d").to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(
    label: &'static str,
    value: String,
    icon: &'static str,
    color: &'static str,
) -> Element {
    rsx! {
        div { class: "bg-dark-800 border border-zinc-800 rounded-xl p-5",
            div { class: "flex items-center justify-between mb-2",
                span { class: "text-xs font-medium text-zinc-500", "{label}" }
                span { class: "text-xl opacity-50 text-zinc-500", "{icon}" }
            }
            div { class: "text-2xl font-bold font-mono {color}", "{value}" }
        }
    }
}

#[component]
fn PipelineStageRow(stage: DealStage, count: usize, value: f64) -> Element {
    let total_pipeline = 600000.0;
    let percentage = ((value / total_pipeline) * 100.0).min(100.0);

    let color_class = stage.tailwind_bg();

    rsx! {
        div { class: "py-3 border-b border-zinc-800 last:border-b-0",
            div { class: "flex items-center justify-between mb-2",
                div { class: "flex items-center gap-2",
                    span { class: "w-2 h-2 rounded-full {color_class}" }
                    span { class: "text-sm font-medium text-zinc-100", "{stage.display_name()}" }
                    span { class: "text-xs text-zinc-500", "({count})" }
                }
                span { class: "font-mono text-sm text-accent", "{format_currency(value)}" }
            }
            // Progress bar
            div { class: "h-1 bg-dark-700 rounded-full overflow-hidden",
                div {
                    class: "h-full {color_class} rounded-full transition-all duration-300",
                    style: "width: {percentage}%",
                }
            }
        }
    }
}

#[component]
fn ActivityRow(icon: String, title: String, meta: String, completed: bool) -> Element {
    let opacity = if completed { "opacity-50" } else { "" };

    rsx! {
        div { class: "flex items-center gap-4 py-3 border-b border-zinc-800 last:border-b-0 {opacity}",
            div { class: "w-8 h-8 rounded-full bg-dark-700 flex items-center justify-center text-sm",
                "{icon}"
            }
            div { class: "flex-1",
                div { class: "text-sm text-zinc-100", "{title}" }
                div { class: "text-xs text-zinc-500", "{meta}" }
            }
        }
    }
}

#[component]
fn TaskRow(id: String, title: String, due: Option<String>) -> Element {
    let mut data = use_app_data();

    rsx! {
        div {
            class: "flex items-center gap-4 py-3 border-b border-zinc-800 last:border-b-0 cursor-pointer hover:bg-dark-700/50 -mx-2 px-2 rounded transition-colors",
            onclick: move |_| {
                crate::state::toggle_activity_completed(&mut data, &id);
            },
            div { class: "w-5 h-5 border-2 border-zinc-600 rounded flex items-center justify-center hover:border-accent transition-colors" }
            div { class: "flex-1",
                div { class: "text-sm text-zinc-100", "{title}" }
                if let Some(d) = due {
                    div { class: "text-xs text-zinc-500", "Due: {d}" }
                }
            }
        }
    }
}

fn format_currency(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("${:.1}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("${:.0}K", value / 1_000.0)
    } else {
        format!("${:.0}", value)
    }
}
