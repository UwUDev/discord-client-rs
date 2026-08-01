use crate::structs::guild::experiment::GuildExperiment;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExperimentAssignments {
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub assignments: Vec<UserExperiment>,
    pub guild_experiments: Option<Vec<GuildExperiment>>,
}

#[derive(Debug, Clone)]
pub struct UserExperiment {
    pub hash: i64,
    pub revision: i64,
    pub bucket: i64,
    pub r#override: bool,
    pub population: i64,
    pub hash_result: i64,
    pub aa_mode: i64,
    pub trigger_debugging: i64,
    pub holdout_name: Option<String>,
    pub holdout_revision: Option<i64>,
    pub holdout_bucket: Option<i64>,
}

impl<'de> Deserialize<'de> for UserExperiment {
    fn deserialize<D>(deserializer: D) -> Result<UserExperiment, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<Value> = Deserialize::deserialize(deserializer)?;
        let get_i64 = |i: usize| v.get(i).and_then(Value::as_i64).unwrap_or_default();
        Ok(UserExperiment {
            hash: get_i64(0),
            revision: get_i64(1),
            bucket: get_i64(2),
            r#override: get_i64(3) == 0,
            population: get_i64(4),
            hash_result: get_i64(5),
            aa_mode: get_i64(6),
            trigger_debugging: get_i64(7),
            holdout_name: v.get(8).and_then(Value::as_str).map(str::to_string),
            holdout_revision: v.get(9).and_then(Value::as_i64),
            holdout_bucket: v.get(10).and_then(Value::as_i64),
        })
    }
}
