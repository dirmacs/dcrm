use crate::models::{Deal, DealStage};
use crate::state::{Modal, delete_deal, update_deal_stage, use_app_data, use_modal};
use dioxus::prelude::*;

#[component]
pub fn DealsPage() -> Element {
    let data = use_app_data();
    let mut modal = use_modal();

    rsx! {
        div { class: "flex-1 overflow-hidden p-6 flex flex-col",
            // Header Stats
            div { class: "flex items-center justify-between mb-6",
                div { class: "flex gap-8",
                    PipelineStat {
                        label: "Total Pipeline",
                        value: format_currency(data.read().total_pipeline_value()),
                    }
                    PipelineStat {
                        label: "Weighted Value",
                        value: format_currency(data.read().weighted_pipeline_value()),
                    }
                    PipelineStat {
                        label: "Active Deals",
                        value: data.read().active_deals_count().to_string(),
                    }
                }
                button {
                    class: "px-4 py-2 bg-accent text-dark-900 text-sm font-medium rounded-md hover:bg-accent-dim transition-colors",
                    onclick: move |_| modal.set(Modal::NewDeal),
                    "+ New Deal"
                }
            }

            // Pipeline Board
            div { class: "flex gap-4 flex-1 overflow-x-auto pb-4",
                for stage in DealStage::active() {
                    PipelineColumn {
                        stage: stage,
                        deals: data.read().deals_by_stage(stage).into_iter().cloned().collect(),
                    }
                }

                // Won & Lost columns (collapsed)
                ClosedDealsColumn {
                    won_deals: data.read().deals_by_stage(DealStage::Won).into_iter().cloned().collect(),
                    lost_deals: data.read().deals_by_stage(DealStage::Lost).into_iter().cloned().collect(),
                }
            }
        }
    }
}

#[component]
fn PipelineStat(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            div { class: "text-xs text-zinc-500 mb-1", "{label}" }
            div { class: "font-mono font-semibold text-lg text-zinc-100", "{value}" }
        }
    }
}

#[component]
fn PipelineColumn(stage: DealStage, deals: Vec<Deal>) -> Element {
    let total_value: f64 = deals.iter().map(|d| d.value).sum();

    let dot_color = stage.tailwind_bg();

    rsx! {
        div { class: "min-w-[280px] w-[280px] bg-dark-800 border border-zinc-800 rounded-xl flex flex-col max-h-full",
            // Column Header
            div { class: "flex items-center justify-between p-4 border-b border-zinc-800",
                div { class: "flex items-center gap-2",
                    span { class: "w-2 h-2 rounded-full {dot_color}" }
                    span { class: "font-medium text-sm text-zinc-100", "{stage.display_name()}" }
                }
                div { class: "flex items-center gap-2",
                    span { class: "text-xs bg-dark-700 px-2 py-0.5 rounded-full text-zinc-500", "{deals.len()}" }
                    span { class: "text-xs text-zinc-500 font-mono", "{format_currency(total_value)}" }
                }
            }

            // Column Body
            div { class: "flex-1 overflow-y-auto p-3 space-y-3",
                for deal in deals {
                    DealCard { deal: deal }
                }
            }
        }
    }
}

#[component]
fn DealCard(deal: Deal) -> Element {
    let data = use_app_data();
    let mut modal = use_modal();
    let deal_id = deal.id.clone();
    let mut show_actions = use_signal(|| false);

    let contact_name = deal
        .contact_id
        .as_ref()
        .and_then(|id| data.read().contact_by_id(id).map(|c| c.full_name()));

    rsx! {
        div {
            class: "bg-dark-700 border border-zinc-700 rounded-lg p-3 cursor-pointer hover:border-zinc-600
                    hover:-translate-y-0.5 hover:shadow-lg transition-all",
            onmouseenter: move |_| show_actions.set(true),
            onmouseleave: move |_| show_actions.set(false),
            onclick: {
                let id = deal_id.clone();
                move |_| modal.set(Modal::DealDetail(id.clone()))
            },

            div { class: "flex items-start justify-between",
                div { class: "font-medium text-sm text-zinc-100 pr-2", "{deal.title}" }
                if *show_actions.read() {
                    DealQuickActions {
                        deal_id: deal_id.clone(),
                        current_stage: deal.stage,
                    }
                }
            }

            div { class: "text-sm text-zinc-500 mt-1 mb-3",
                "{deal.company}"
                if let Some(name) = contact_name {
                    span { class: "text-zinc-600", " • {name}" }
                }
            }

            div { class: "flex items-center justify-between",
                span { class: "font-mono text-sm font-semibold text-accent", "{deal.format_value()}" }
                span { class: "text-xs text-zinc-500", "{deal.probability}% prob." }
            }
        }
    }
}

#[component]
fn DealQuickActions(deal_id: String, current_stage: DealStage) -> Element {
    let mut data = use_app_data();
    let mut show_menu = use_signal(|| false);

    let next_stages: Vec<DealStage> = DealStage::all()
        .into_iter()
        .filter(|s| *s != current_stage)
        .collect();

    rsx! {
        div {
            class: "relative",
            onclick: move |e| {
                e.stop_propagation();
                let current = *show_menu.read();
                show_menu.set(!current);
            },

            button {
                class: "w-6 h-6 flex items-center justify-center rounded text-zinc-400 hover:bg-zinc-600 hover:text-zinc-100 transition-colors",
                "⋮"
            }

            if *show_menu.read() {
                div {
                    class: "absolute right-0 top-full mt-1 bg-dark-600 border border-zinc-700 rounded-lg min-w-[160px] shadow-xl z-10 py-1",

                    div { class: "text-xs text-zinc-500 px-3 py-1", "Move to:" }

                    for stage in next_stages {
                        {
                            let dot_color = stage.tailwind_bg();

                            rsx! {
                                div {
                                    class: "flex items-center gap-2 px-3 py-2 text-sm text-zinc-300 hover:bg-zinc-700 cursor-pointer transition-colors",
                                    onclick: {
                                        let id = deal_id.clone();
                                        move |e| {
                                            e.stop_propagation();
                                            update_deal_stage(&mut data, &id, stage);
                                            show_menu.set(false);
                                        }
                                    },
                                    span { class: "w-1.5 h-1.5 rounded-full {dot_color}" }
                                    "{stage.display_name()}"
                                }
                            }
                        }
                    }

                    div { class: "border-t border-zinc-700 my-1" }

                    div {
                        class: "flex items-center gap-2 px-3 py-2 text-sm text-red-400 hover:bg-red-500/10 cursor-pointer transition-colors",
                        onclick: {
                            let id = deal_id.clone();
                            move |e| {
                                e.stop_propagation();
                                delete_deal(&mut data, &id);
                            }
                        },
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
fn ClosedDealsColumn(won_deals: Vec<Deal>, lost_deals: Vec<Deal>) -> Element {
    let won_value: f64 = won_deals.iter().map(|d| d.value).sum();
    let lost_value: f64 = lost_deals.iter().map(|d| d.value).sum();

    rsx! {
        div { class: "min-w-[280px] w-[280px] bg-dark-700 border border-zinc-800 rounded-xl flex flex-col",
            // Won Section
            div { class: "flex items-center justify-between p-4",
                div { class: "flex items-center gap-2",
                    span { class: "w-2 h-2 rounded-full bg-emerald-500" }
                    span { class: "font-medium text-sm text-emerald-400", "Won" }
                }
                div { class: "flex items-center gap-2",
                    span { class: "text-xs bg-dark-600 px-2 py-0.5 rounded-full text-zinc-500", "{won_deals.len()}" }
                    span { class: "text-xs text-emerald-400 font-mono", "{format_currency(won_value)}" }
                }
            }

            // Lost Section
            div { class: "flex items-center justify-between p-4 border-t border-zinc-800",
                div { class: "flex items-center gap-2",
                    span { class: "w-2 h-2 rounded-full bg-red-500" }
                    span { class: "font-medium text-sm text-red-400", "Lost" }
                }
                div { class: "flex items-center gap-2",
                    span { class: "text-xs bg-dark-600 px-2 py-0.5 rounded-full text-zinc-500", "{lost_deals.len()}" }
                    span { class: "text-xs text-red-400 font-mono", "{format_currency(lost_value)}" }
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
