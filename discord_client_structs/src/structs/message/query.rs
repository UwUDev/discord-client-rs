use derive_builder::Builder;

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageQuery {
    pub around: Option<u64>,
    pub before: Option<u64>,
    pub after: Option<u64>,
    pub limit: u8,
}
impl MessageQuery {
    pub fn to_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(around) = self.around {
            map.insert("around".to_string(), around.to_string());
        }
        if let Some(before) = self.before {
            map.insert("before".to_string(), before.to_string());
        }
        if let Some(after) = self.after {
            map.insert("after".to_string(), after.to_string());
        }
        let limit = if self.limit == 0u8 || self.limit > 100u8 {
            50u8
        } else {
            self.limit
        };
        map.insert("limit".to_string(), limit.to_string());
        map
    }
}