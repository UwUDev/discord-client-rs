use serde::{Deserialize, Deserializer};

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
}

impl<'de> Deserialize<'de> for UserExperiment {
    fn deserialize<D>(deserializer: D) -> Result<UserExperiment, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<i64> = Deserialize::deserialize(deserializer)?;
        Ok(UserExperiment {
            hash: v.get(0).cloned().unwrap_or_default(),
            revision: v.get(1).cloned().unwrap_or_default(),
            bucket: v.get(2).cloned().unwrap_or_default(),
            r#override: v.get(3).cloned().unwrap_or_default() == 0,
            population: v.get(4).cloned().unwrap_or_default(),
            hash_result: v.get(5).cloned().unwrap_or_default(),
            aa_mode: v.get(6).cloned().unwrap_or_default(),
            trigger_debugging: v.get(7).cloned().unwrap_or_default(),
        })
    }
}
