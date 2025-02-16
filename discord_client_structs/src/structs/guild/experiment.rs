// Guild Experiment Structure
//
// This object is represented as an array of the following fields:
//
// Field	Type	Description
// hash	integer	32-bit unsigned Murmur3 hash of the experiment's name
// hash_key 1	?string	A human-readable experiment name (formatted as year-month_name) to use for hashing calculations, prioritized over the client name
// revision	integer	Current version of the rollout
// populations	array[experiment population object]	The experiment rollout's populations
// overrides 2	array[experiment bucket override object]	Specific bucket overrides for the experiment
// overrides_formatted 2	array[array[experiment population object]]	Populations of overrides for the experiment
// holdout_name 3	?string	The human-readable experiment name (formatted as year-month_name) that the experiment is dependent on
// holdout_bucket 3	?integer	The required bucket for the experiment the experiment is dependent on
// aa_mode 2	integer	The experiment's A/A testing mode, represented as an integer-casted boolean
// trigger_debugging	integer	Whether the experiment's analytics trigger debugging is enabled, represented as an integer-casted boolean

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
            hash: v.get(0).cloned().unwrap_or_default().as_i64().unwrap_or_default(),
            hash_key: v.get(1).cloned().unwrap_or_default().as_str().map(|s| s.to_string()),
            revision: v.get(2).cloned().unwrap_or_default().as_i64().unwrap_or_default(),
            populations: v.get(3).cloned().unwrap_or_default().as_array().map(|a| a.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect()).unwrap_or_default(),
            overrides: v.get(4).cloned().unwrap_or_default().as_array().map(|a| a.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect()).unwrap_or_default(),
            overrides_formatted: v.get(5).cloned().unwrap_or_default().as_array().map(|a| a.iter().map(|v| v.as_array().map(|a| a.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect()).unwrap_or_default()).collect()).unwrap_or_default(),
            holdout_name: v.get(6).cloned().unwrap_or_default().as_str().map(|s| s.to_string()),
            holdout_bucket: v.get(7).cloned().unwrap_or_default().as_i64().map(|i| i),
            aa_mode: v.get(8).cloned().unwrap_or_default().as_i64().unwrap_or_default(),
            trigger_debugging: v.get(9).cloned().unwrap_or_default().as_i64().unwrap_or_default(),
        })
    }
}

// Experiment Population Structure
//
// This object is represented as an array of the following fields:
//
// Field	Type	Description
// ranges	array[experiment population range object]	The ranges for this population
// filters	experiment population filters object	The filters that the resource must satisfy to be in this population

#[derive(Debug, Clone)]
pub struct ExperimentPopulation {
    pub ranges: Vec<ExperimentPopulationRange>,
    //pub filters: ExperimentPopulationFilters,
}

impl<'de> Deserialize<'de> for ExperimentPopulation {
    fn deserialize<D>(deserializer: D) -> Result<ExperimentPopulation, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
        Ok(ExperimentPopulation {
            ranges: v.get(0).cloned().unwrap_or_default().as_array().map(|a| a.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect()).unwrap_or_default(),
            //filters: serde_json::from_value(v.get(1).cloned().unwrap_or_default()).unwrap(),
        })
    }
}

// Experiment Population Range Structure
//
// This object is represented as an array of the following fields:
//
// Field	Type	Description
// bucket	integer	The bucket this range grants
// rollout	array[experiment population rollout object]	The range rollout
// Experiment Population Rollout Structure
//
// Field	Type	Description
// s	integer	The start of this range
// e	integer	The end of this range

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
            bucket: v.get(0).cloned().unwrap_or_default().as_i64().unwrap_or_default(),
            rollout: v.get(1).cloned().unwrap_or_default().as_array().map(|a| a.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect()).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Deserialize, Clone)] // no need for custom deserialization
pub struct ExperimentPopulationRollout {
    pub s: i64,
    pub e: i64,
}

// Experiment Bucket Override Structure
//
// Field	Type	Description
// b	integer	Bucket assigned to these resources
// k	array[snowflake]	Resources granted access to this bucket

#[derive(Debug, Deserialize, Clone)] // no need for custom deserialization
pub struct ExperimentBucketOverride {
    pub b: i64,
    pub k: Vec<String>,
}