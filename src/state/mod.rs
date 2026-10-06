use crate::models::{Activity, AppData, Contact, Deal, DealStage, save_data};
use dioxus::prelude::*;

// ============================================================================
// Global App State
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Dashboard,
    Contacts,
    Deals,
    Activities,
}

impl Default for View {
    fn default() -> Self {
        View::Dashboard
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modal {
    None,
    NewContact,
    EditContact(String),
    NewDeal,
    EditDeal(String),
    NewActivity,
    ContactDetail(String),
    DealDetail(String),
    Search,
}

impl Default for Modal {
    fn default() -> Self {
        Modal::None
    }
}

// ============================================================================
// App State Context
// ============================================================================

pub fn use_app_data() -> Signal<AppData> {
    use_context::<Signal<AppData>>()
}

pub fn use_current_view() -> Signal<View> {
    use_context::<Signal<View>>()
}

pub fn use_modal() -> Signal<Modal> {
    use_context::<Signal<Modal>>()
}

pub fn use_search_query() -> Signal<String> {
    use_context::<Signal<String>>()
}

// ============================================================================
// Actions
// ============================================================================

pub fn add_contact(data: &mut Signal<AppData>, contact: Contact) {
    data.write().contacts.push(contact);
    let _ = save_data(&data.read());
}

pub fn update_contact(data: &mut Signal<AppData>, contact: Contact) {
    if let Some(existing) = data
        .write()
        .contacts
        .iter_mut()
        .find(|c| c.id == contact.id)
    {
        *existing = contact;
    }
    let _ = save_data(&data.read());
}

pub fn delete_contact(data: &mut Signal<AppData>, id: &str) {
    data.write().contacts.retain(|c| c.id != id);
    let _ = save_data(&data.read());
}

pub fn add_deal(data: &mut Signal<AppData>, deal: Deal) {
    data.write().deals.push(deal);
    let _ = save_data(&data.read());
}

pub fn update_deal(data: &mut Signal<AppData>, deal: Deal) {
    if let Some(existing) = data.write().deals.iter_mut().find(|d| d.id == deal.id) {
        *existing = deal;
    }
    let _ = save_data(&data.read());
}

pub fn update_deal_stage(data: &mut Signal<AppData>, deal_id: &str, new_stage: DealStage) {
    if let Some(deal) = data.write().deals.iter_mut().find(|d| d.id == deal_id) {
        deal.stage = new_stage;
        deal.updated_at = chrono::Utc::now();

        // Update probability based on stage
        deal.probability = match new_stage {
            DealStage::Lead => 10,
            DealStage::Qualified => 25,
            DealStage::Proposal => 50,
            DealStage::Negotiation => 75,
            DealStage::Won => 100,
            DealStage::Lost => 0,
            // Dsprint pipeline stages have no established close-probability yet.
            DealStage::DsprintSubmitted => 60,
            DealStage::DsprintActivated => 80,
            DealStage::FirstAgentRun => 90,
        };
    }
    let _ = save_data(&data.read());
}

pub fn delete_deal(data: &mut Signal<AppData>, id: &str) {
    data.write().deals.retain(|d| d.id != id);
    let _ = save_data(&data.read());
}

pub fn add_activity(data: &mut Signal<AppData>, activity: Activity) {
    data.write().activities.push(activity);
    let _ = save_data(&data.read());
}

pub fn update_activity(data: &mut Signal<AppData>, activity: Activity) {
    if let Some(existing) = data
        .write()
        .activities
        .iter_mut()
        .find(|a| a.id == activity.id)
    {
        *existing = activity;
    }
    let _ = save_data(&data.read());
}

pub fn toggle_activity_completed(data: &mut Signal<AppData>, id: &str) {
    if let Some(activity) = data.write().activities.iter_mut().find(|a| a.id == id) {
        activity.completed = !activity.completed;
        activity.updated_at = chrono::Utc::now();
    }
    let _ = save_data(&data.read());
}

pub fn delete_activity(data: &mut Signal<AppData>, id: &str) {
    data.write().activities.retain(|a| a.id != id);
    let _ = save_data(&data.read());
}

// ============================================================================
// Search Functionality
// ============================================================================

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    Contact(Contact),
    Deal(Deal),
    Activity(Activity),
}

impl SearchResult {
    pub fn title(&self) -> String {
        match self {
            SearchResult::Contact(c) => c.full_name(),
            SearchResult::Deal(d) => d.title.clone(),
            SearchResult::Activity(a) => a.title.clone(),
        }
    }

    pub fn subtitle(&self) -> String {
        match self {
            SearchResult::Contact(c) => c.company.clone().unwrap_or_else(|| c.email.clone()),
            SearchResult::Deal(d) => format!("{} • {}", d.company, d.format_value()),
            SearchResult::Activity(a) => a.activity_type.display_name().to_string(),
        }
    }

    pub fn result_type(&self) -> &str {
        match self {
            SearchResult::Contact(_) => "Contact",
            SearchResult::Deal(_) => "Deal",
            SearchResult::Activity(_) => "Activity",
        }
    }
}

pub fn search(data: &AppData, query: &str) -> Vec<SearchResult> {
    if query.is_empty() {
        return Vec::new();
    }

    let matcher = SkimMatcherV2::default();
    let mut results: Vec<(i64, SearchResult)> = Vec::new();

    // Search contacts
    for contact in &data.contacts {
        let search_str = format!(
            "{} {} {} {}",
            contact.first_name,
            contact.last_name,
            contact.email,
            contact.company.as_deref().unwrap_or("")
        );
        if let Some(score) = matcher.fuzzy_match(&search_str, query) {
            results.push((score, SearchResult::Contact(contact.clone())));
        }
    }

    // Search deals
    for deal in &data.deals {
        let search_str = format!("{} {}", deal.title, deal.company);
        if let Some(score) = matcher.fuzzy_match(&search_str, query) {
            results.push((score, SearchResult::Deal(deal.clone())));
        }
    }

    // Search activities
    for activity in &data.activities {
        let search_str = format!(
            "{} {}",
            activity.title,
            activity.description.as_deref().unwrap_or("")
        );
        if let Some(score) = matcher.fuzzy_match(&search_str, query) {
            results.push((score, SearchResult::Activity(activity.clone())));
        }
    }

    // Sort by score descending
    results.sort_by(|a, b| b.0.cmp(&a.0));

    // Return top 10 results
    results.into_iter().take(10).map(|(_, r)| r).collect()
}
