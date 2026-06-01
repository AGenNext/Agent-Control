use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DriftCategory {
    CapabilityDrift,
    ToolDrift,
    PromptDrift,
    MemoryDrift,
    KnowledgeDrift,
    WorkflowDrift,
    PolicyDrift,
    ObjectiveDrift,
    ModelDrift,
    ConfigurationDrift,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DriftSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    pub agent_id: String,
    pub version: String,
    #[serde(default)]
    pub skills: BTreeSet<String>,
    #[serde(default)]
    pub tools: BTreeSet<String>,
    #[serde(default)]
    pub prompts: BTreeSet<String>,
    #[serde(default)]
    pub workflows: BTreeSet<String>,
    #[serde(default)]
    pub policies: BTreeSet<String>,
    #[serde(default)]
    pub objectives: BTreeSet<String>,
    #[serde(default)]
    pub constraints: BTreeSet<String>,
    pub model: Option<String>,
    pub knowledge_version: Option<String>,
    pub memory_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSnapshot {
    pub agent_id: String,
    pub captured_at: DateTime<Utc>,
    #[serde(default)]
    pub skills: BTreeSet<String>,
    #[serde(default)]
    pub tools: BTreeSet<String>,
    #[serde(default)]
    pub prompts: BTreeSet<String>,
    #[serde(default)]
    pub workflows: BTreeSet<String>,
    #[serde(default)]
    pub policies: BTreeSet<String>,
    #[serde(default)]
    pub objectives: BTreeSet<String>,
    #[serde(default)]
    pub constraints: BTreeSet<String>,
    pub model: Option<String>,
    pub knowledge_version: Option<String>,
    pub memory_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftCompareRequest {
    pub desired: AgentDefinition,
    pub actual: AgentSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftCompareResponse {
    pub agent_id: String,
    pub overall_score: u16,
    pub highest_severity: DriftSeverity,
    pub drift_count: usize,
    pub drifts: Vec<AgentDrift>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDrift {
    pub id: Uuid,
    pub agent_id: String,
    pub category: DriftCategory,
    pub severity: DriftSeverity,
    pub score: u16,
    pub desired: serde_json::Value,
    pub actual: serde_json::Value,
    pub explanation: String,
    pub created_at: DateTime<Utc>,
}
