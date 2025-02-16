use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct GuildExperiment {
    pub hash: i64,
    pub hash_key: Option<String>,
    pub revision: i64,
    pub populations: Vec<ExperimentPopulation>,
    pub overrides: Vec<ExperimentBucketOverride>,
    pub overrides_formatted: Vec<Vec<ExperimentPopulation>>,
    pub holdout_name: Option<String>,
    pub holdout_bucket: Option<i64>,
    pub aa_mode: i64,
    pub trigger_debugging: i64,
}

impl<'de> Deserialize<'de> for GuildExperiment {
    fn deserialize<D>(deserializer: D) -> Result<GuildExperiment, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
        Ok(GuildExperiment {
            hash: v
                .get(0)
                .cloned()
                .unwrap_or_default()
                .as_i64()
                .unwrap_or_default(),
            hash_key: v
                .get(1)
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(|s| s.to_string()),
            revision: v
                .get(2)
                .cloned()
                .unwrap_or_default()
                .as_i64()
                .unwrap_or_default(),
            populations: v
                .get(3)
                .cloned()
                .unwrap_or_default()
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|v| serde_json::from_value(v.clone()).unwrap())
                        .collect()
                })
                .unwrap_or_default(),
            overrides: v
                .get(4)
                .cloned()
                .unwrap_or_default()
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|v| serde_json::from_value(v.clone()).unwrap())
                        .collect()
                })
                .unwrap_or_default(),
            overrides_formatted: v
                .get(5)
                .cloned()
                .unwrap_or_default()
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|v| {
                            v.as_array()
                                .map(|a| {
                                    a.iter()
                                        .map(|v| serde_json::from_value(v.clone()).unwrap())
                                        .collect()
                                })
                                .unwrap_or_default()
                        })
                        .collect()
                })
                .unwrap_or_default(),
            holdout_name: v
                .get(6)
                .cloned()
                .unwrap_or_default()
                .as_str()
                .map(|s| s.to_string()),
            holdout_bucket: v.get(7).cloned().unwrap_or_default().as_i64().map(|i| i),
            aa_mode: v
                .get(8)
                .cloned()
                .unwrap_or_default()
                .as_i64()
                .unwrap_or_default(),
            trigger_debugging: v
                .get(9)
                .cloned()
                .unwrap_or_default()
                .as_i64()
                .unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExperimentPopulation {
    pub ranges: Vec<ExperimentPopulationRange>,
    //pub filters: ExperimentPopulationFilters, // todo: understand this
}

impl<'de> Deserialize<'de> for ExperimentPopulation {
    fn deserialize<D>(deserializer: D) -> Result<ExperimentPopulation, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
        Ok(ExperimentPopulation {
            ranges: v
                .get(0)
                .cloned()
                .unwrap_or_default()
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|v| serde_json::from_value(v.clone()).unwrap())
                        .collect()
                })
                .unwrap_or_default(),
            //filters: serde_json::from_value(v.get(1).cloned().unwrap_or_default()).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExperimentPopulationRange {
    pub bucket: i64,
    pub rollout: Vec<ExperimentPopulationRollout>,
}

impl<'de> Deserialize<'de> for ExperimentPopulationRange {
    fn deserialize<D>(deserializer: D) -> Result<ExperimentPopulationRange, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
        Ok(ExperimentPopulationRange {
            bucket: v
                .get(0)
                .cloned()
                .unwrap_or_default()
                .as_i64()
                .unwrap_or_default(),
            rollout: v
                .get(1)
                .cloned()
                .unwrap_or_default()
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|v| serde_json::from_value(v.clone()).unwrap())
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

#[derive(Debug, Deserialize, Clone)] // no need for custom deserialization
pub struct ExperimentPopulationRollout {
    pub s: i64,
    pub e: i64,
}

#[derive(Debug, Deserialize, Clone)] // no need for custom deserialization
pub struct ExperimentBucketOverride {
    pub b: i64,
    pub k: Vec<String>,
}
