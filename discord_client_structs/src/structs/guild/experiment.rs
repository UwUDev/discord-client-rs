use derive_builder::Builder;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
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

impl Serialize for GuildExperiment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(10))?;
        seq.serialize_element(&self.hash)?;
        seq.serialize_element(&self.hash_key)?;
        seq.serialize_element(&self.revision)?;
        seq.serialize_element(&self.populations)?;
        seq.serialize_element(&self.overrides)?;
        seq.serialize_element(&self.overrides_formatted)?;
        seq.serialize_element(&self.holdout_name)?;
        seq.serialize_element(&self.holdout_bucket)?;
        seq.serialize_element(&self.aa_mode)?;
        seq.serialize_element(&self.trigger_debugging)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
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

impl Serialize for ExperimentPopulation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1))?;
        seq.serialize_element(&self.ranges)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
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

impl Serialize for ExperimentPopulationRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.bucket)?;
        seq.serialize_element(&self.rollout)?;
        seq.end()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)] // no need for custom deserialization
pub struct ExperimentPopulationRollout {
    pub s: i64,
    pub e: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)] // no need for custom deserialization
pub struct ExperimentBucketOverride {
    pub b: i64,
    pub k: Vec<String>,
}
