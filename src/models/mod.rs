use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Contact Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub position: Option<String>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(first_name: String, last_name: String, email: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            first_name,
            last_name,
            email,
            phone: None,
            company: None,
            position: None,
            tags: Vec::new(),
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn initials(&self) -> String {
        let first = self.first_name.chars().next().unwrap_or('?');
        let last = self.last_name.chars().next().unwrap_or('?');

        format!("{}{}", first, last).to_uppercase()
    }
}

impl Default for Contact {
    fn default() -> Self {
        Self::new(String::new(), String::new(), String::new())
    }
}

// ============================================================================
// Deal Model
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DealStage {
    Lead,
    Qualified,
    Proposal,
    Negotiation,
    Won,
    Lost,
    DsprintSubmitted,
    DsprintActivated,
    FirstAgentRun,
}

impl DealStage {
    pub fn all() -> Vec<DealStage> {
        vec![
            DealStage::Lead,
            DealStage::Qualified,
            DealStage::Proposal,
            DealStage::Negotiation,
            DealStage::Won,
            DealStage::Lost,
            DealStage::DsprintSubmitted,
            DealStage::DsprintActivated,
            DealStage::FirstAgentRun,
        ]
    }

    pub fn active() -> Vec<DealStage> {
        vec![
            DealStage::Lead,
            DealStage::Qualified,
            DealStage::Proposal,
            DealStage::Negotiation,
        ]
    }

    pub fn display_name(&self) -> &str {
        match self {
            DealStage::Lead => "Lead",
            DealStage::Qualified => "Qualified",
            DealStage::Proposal => "Proposal",
            DealStage::Negotiation => "Negotiation",
            DealStage::Won => "Won",
            DealStage::Lost => "Lost",
            DealStage::DsprintSubmitted => "dsprint_submitted",
            DealStage::DsprintActivated => "dsprint_activated",
            DealStage::FirstAgentRun => "first_agent_run",
        }
    }

    pub fn color(&self) -> &str {
        match self {
            DealStage::Lead => "#3b82f6",
            DealStage::Qualified => "#8b5cf6",
            DealStage::Proposal => "#f59e0b",
            DealStage::Negotiation => "#ec4899",
            DealStage::Won => "#10b981",
            DealStage::Lost => "#ef4444",
            DealStage::DsprintSubmitted => "#6366f1",
            DealStage::DsprintActivated => "#8b5cf6",
            DealStage::FirstAgentRun => "#0ea5e9",
        }
    }

    pub fn badge_class(&self) -> &str {
        match self {
            DealStage::Lead => "badge-lead",
            DealStage::Qualified => "badge-qualified",
            DealStage::Proposal => "badge-proposal",
            DealStage::Negotiation => "badge-negotiation",
            DealStage::Won => "badge-won",
            DealStage::Lost => "badge-lost",
            DealStage::DsprintSubmitted => "badge-dsprint-submitted",
            DealStage::DsprintActivated => "badge-dsprint-activated",
            DealStage::FirstAgentRun => "badge-first-agent-run",
        }
    }
}

impl std::fmt::Display for DealStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Deal {
    pub id: String,
    pub title: String,
    pub contact_id: Option<String>,
    pub company: String,
    pub value: f64,
    pub stage: DealStage,
    pub probability: u8,
    pub expected_close: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub pipeline: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Deal {
    pub fn new(title: String, company: String, value: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            contact_id: None,
            company,
            value,
            stage: DealStage::Lead,
            probability: 10,
            expected_close: None,
            notes: None,
            pipeline: "manual_sales".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn weighted_value(&self) -> f64 {
        self.value * (self.probability as f64 / 100.0)
    }

    pub fn format_value(&self) -> String {
        if self.value >= 1_000_000.0 {
            format!("${:.1}M", self.value / 1_000_000.0)
        } else if self.value >= 1_000.0 {
            format!("${:.0}K", self.value / 1_000.0)
        } else {
            format!("${:.0}", self.value)
        }
    }
}

impl Default for Deal {
    fn default() -> Self {
        Self::new(String::new(), String::new(), 0.0)
    }
}

// ============================================================================
// Activity Model
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivityType {
    Note,
    Call,
    Email,
    Meeting,
    Task,
}

impl ActivityType {
    pub fn display_name(&self) -> &str {
        match self {
            ActivityType::Note => "Note",
            ActivityType::Call => "Call",
            ActivityType::Email => "Email",
            ActivityType::Meeting => "Meeting",
            ActivityType::Task => "Task",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            ActivityType::Note => "📝",
            ActivityType::Call => "📞",
            ActivityType::Email => "✉️",
            ActivityType::Meeting => "👥",
            ActivityType::Task => "✓",
        }
    }
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Activity {
    pub id: String,
    pub activity_type: ActivityType,
    pub title: String,
    pub description: Option<String>,
    pub contact_id: Option<String>,
    pub deal_id: Option<String>,
    pub completed: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Activity {
    pub fn new(activity_type: ActivityType, title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            activity_type,
            title,
            description: None,
            contact_id: None,
            deal_id: None,
            completed: false,
            due_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn format_date(&self) -> String {
        self.created_at.format("%b %d, %Y").to_string()
    }

    pub fn format_time(&self) -> String {
        self.created_at.format("%H:%M").to_string()
    }
}

impl Default for Activity {
    fn default() -> Self {
        Self::new(ActivityType::Note, String::new())
    }
}

// ============================================================================
// App Data Store
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub contacts: Vec<Contact>,
    pub deals: Vec<Deal>,
    pub activities: Vec<Activity>,
}

impl AppData {
    pub fn new() -> Self {
        Self::default()
    }

    // Statistics
    pub fn total_pipeline_value(&self) -> f64 {
        self.deals
            .iter()
            .filter(|d| {
                matches!(
                    d.stage,
                    DealStage::Lead
                        | DealStage::Qualified
                        | DealStage::Proposal
                        | DealStage::Negotiation
                )
            })
            .map(|d| d.value)
            .sum()
    }

    pub fn weighted_pipeline_value(&self) -> f64 {
        self.deals
            .iter()
            .filter(|d| {
                matches!(
                    d.stage,
                    DealStage::Lead
                        | DealStage::Qualified
                        | DealStage::Proposal
                        | DealStage::Negotiation
                )
            })
            .map(|d| d.weighted_value())
            .sum()
    }

    pub fn won_deals_value(&self) -> f64 {
        self.deals
            .iter()
            .filter(|d| d.stage == DealStage::Won)
            .map(|d| d.value)
            .sum()
    }

    pub fn deals_by_stage(&self, stage: DealStage) -> Vec<&Deal> {
        self.deals.iter().filter(|d| d.stage == stage).collect()
    }

    pub fn active_deals_count(&self) -> usize {
        self.deals
            .iter()
            .filter(|d| {
                matches!(
                    d.stage,
                    DealStage::Lead
                        | DealStage::Qualified
                        | DealStage::Proposal
                        | DealStage::Negotiation
                )
            })
            .count()
    }

    pub fn pending_tasks_count(&self) -> usize {
        self.activities
            .iter()
            .filter(|a| a.activity_type == ActivityType::Task && !a.completed)
            .count()
    }

    pub fn contact_by_id(&self, id: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.id == id)
    }

    pub fn deal_by_id(&self, id: &str) -> Option<&Deal> {
        self.deals.iter().find(|d| d.id == id)
    }

    pub fn activities_for_contact(&self, contact_id: &str) -> Vec<&Activity> {
        self.activities
            .iter()
            .filter(|a| a.contact_id.as_deref() == Some(contact_id))
            .collect()
    }

    pub fn activities_for_deal(&self, deal_id: &str) -> Vec<&Activity> {
        self.activities
            .iter()
            .filter(|a| a.deal_id.as_deref() == Some(deal_id))
            .collect()
    }

    pub fn recent_activities(&self, limit: usize) -> Vec<&Activity> {
        let mut activities: Vec<&Activity> = self.activities.iter().collect();
        activities.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        activities.into_iter().take(limit).collect()
    }

    // Sample data for demo
    pub fn with_sample_data() -> Self {
        let mut data = Self::new();

        // Sample Contacts
        let contacts = vec![
            {
                let mut c = Contact::new(
                    "Sarah".into(),
                    "Chen".into(),
                    "sarah.chen@techcorp.com".into(),
                );
                c.company = Some("TechCorp Solutions".into());
                c.position = Some("VP of Engineering".into());
                c.phone = Some("+1 (555) 123-4567".into());
                c.tags = vec!["enterprise".into(), "hot-lead".into()];
                c
            },
            {
                let mut c = Contact::new(
                    "Marcus".into(),
                    "Johnson".into(),
                    "m.johnson@innovate.io".into(),
                );
                c.company = Some("Innovate.io".into());
                c.position = Some("CTO".into());
                c.phone = Some("+1 (555) 234-5678".into());
                c.tags = vec!["startup".into(), "referral".into()];
                c
            },
            {
                let mut c = Contact::new(
                    "Emily".into(),
                    "Rodriguez".into(),
                    "emily.r@globalfinance.com".into(),
                );
                c.company = Some("Global Finance Inc".into());
                c.position = Some("Director of Operations".into());
                c.phone = Some("+1 (555) 345-6789".into());
                c.tags = vec!["enterprise".into(), "finance".into()];
                c
            },
            {
                let mut c = Contact::new(
                    "David".into(),
                    "Kim".into(),
                    "david.kim@startupxyz.com".into(),
                );
                c.company = Some("StartupXYZ".into());
                c.position = Some("Founder & CEO".into());
                c.phone = Some("+1 (555) 456-7890".into());
                c.tags = vec!["startup".into(), "founder".into()];
                c
            },
            {
                let mut c = Contact::new(
                    "Lisa".into(),
                    "Thompson".into(),
                    "lisa.t@medtech.health".into(),
                );
                c.company = Some("MedTech Health".into());
                c.position = Some("Head of Product".into());
                c.phone = Some("+1 (555) 567-8901".into());
                c.tags = vec!["healthcare".into(), "enterprise".into()];
                c
            },
        ];

        // Sample Deals
        let deals = vec![
            {
                let mut d = Deal::new(
                    "Enterprise Platform License".into(),
                    "TechCorp Solutions".into(),
                    150000.0,
                );
                d.contact_id = Some(contacts[0].id.clone());
                d.stage = DealStage::Negotiation;
                d.probability = 75;
                d
            },
            {
                let mut d = Deal::new(
                    "API Integration Package".into(),
                    "Innovate.io".into(),
                    45000.0,
                );
                d.contact_id = Some(contacts[1].id.clone());
                d.stage = DealStage::Proposal;
                d.probability = 50;
                d
            },
            {
                let mut d = Deal::new(
                    "Financial Analytics Suite".into(),
                    "Global Finance Inc".into(),
                    280000.0,
                );
                d.contact_id = Some(contacts[2].id.clone());
                d.stage = DealStage::Qualified;
                d.probability = 30;
                d
            },
            {
                let mut d = Deal::new(
                    "Startup Growth Package".into(),
                    "StartupXYZ".into(),
                    25000.0,
                );
                d.contact_id = Some(contacts[3].id.clone());
                d.stage = DealStage::Lead;
                d.probability = 15;
                d
            },
            {
                let mut d = Deal::new(
                    "Healthcare Compliance Module".into(),
                    "MedTech Health".into(),
                    95000.0,
                );
                d.contact_id = Some(contacts[4].id.clone());
                d.stage = DealStage::Proposal;
                d.probability = 60;
                d
            },
            {
                let mut d = Deal::new(
                    "Consulting Engagement Q1".into(),
                    "TechCorp Solutions".into(),
                    50000.0,
                );
                d.contact_id = Some(contacts[0].id.clone());
                d.stage = DealStage::Won;
                d.probability = 100;
                d
            },
        ];

        // Sample Activities
        let activities = vec![
            {
                let mut a =
                    Activity::new(ActivityType::Meeting, "Discovery call with Sarah".into());
                a.contact_id = Some(contacts[0].id.clone());
                a.deal_id = Some(deals[0].id.clone());
                a.description = Some("Discussed enterprise requirements and timeline".into());
                a.completed = true;
                a
            },
            {
                let mut a = Activity::new(ActivityType::Email, "Sent proposal to Marcus".into());
                a.contact_id = Some(contacts[1].id.clone());
                a.deal_id = Some(deals[1].id.clone());
                a.description = Some("API integration proposal with pricing tiers".into());
                a.completed = true;
                a
            },
            {
                let mut a = Activity::new(
                    ActivityType::Task,
                    "Follow up with Emily on requirements".into(),
                );
                a.contact_id = Some(contacts[2].id.clone());
                a.deal_id = Some(deals[2].id.clone());
                a.completed = false;
                a
            },
            {
                let mut a = Activity::new(ActivityType::Call, "Intro call with David".into());
                a.contact_id = Some(contacts[3].id.clone());
                a.deal_id = Some(deals[3].id.clone());
                a.description = Some("Initial discussion about startup needs".into());
                a.completed = true;
                a
            },
            {
                let mut a = Activity::new(
                    ActivityType::Note,
                    "Lisa mentioned budget approval pending".into(),
                );
                a.contact_id = Some(contacts[4].id.clone());
                a.deal_id = Some(deals[4].id.clone());
                a.completed = false;
                a
            },
            {
                let mut a = Activity::new(ActivityType::Task, "Prepare demo for TechCorp".into());
                a.contact_id = Some(contacts[0].id.clone());
                a.deal_id = Some(deals[0].id.clone());
                a.completed = false;
                a
            },
        ];

        data.contacts = contacts;
        data.deals = deals;
        data.activities = activities;
        data
    }
}

// ============================================================================
// Data Persistence
// ============================================================================

use std::fs;
use std::path::PathBuf;

fn get_data_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("dcrm");
    fs::create_dir_all(&path).ok();
    path.push("data.json");
    path
}

pub fn save_data(data: &AppData) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_data_path();
    let json = serde_json::to_string_pretty(data)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_data() -> AppData {
    let path = get_data_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_else(|_| AppData::with_sample_data()),
            Err(_) => AppData::with_sample_data(),
        }
    } else {
        AppData::with_sample_data()
    }
}
