use derive_builder::Builder;

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageQuery {
    pub around: Option<u64>,
    pub before: Option<u64>,
    pub after: Option<u64>,
    #[builder(default = "50")]
    pub limit: u8,
}

impl MessageQuery {
    pub fn to_map(&self) -> std::collections::HashMap<&str, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(around) = self.around {
            map.insert("around", around.to_string());
        }
        if let Some(before) = self.before {
            map.insert("before", before.to_string());
        }
        if let Some(after) = self.after {
            map.insert("after", after.to_string());
        }
        map.insert("limit", self.limit.to_string());
        map
    }
}