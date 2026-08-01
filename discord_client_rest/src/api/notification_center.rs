use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct NotificationCenterRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> NotificationCenterRest<'a> {
    pub async fn get_items(&self, limit: u16) -> BoxedResult<Value> {
        let mut query = std::collections::HashMap::new();
        query.insert("limit".to_string(), limit.to_string());

        self.client
            .get(
                "users/@me/notification-center/items",
                Some(query),
                Some(RequestProperties::home()),
            )
            .await
    }

    pub async fn acknowledge_item(&self, item_id: u64) -> BoxedResult<Value> {
        self.client
            .post::<Value, ()>(
                &format!("users/@me/notification-center/items/{}/ack", item_id),
                None::<()>,
                Some(RequestProperties::home()),
            )
            .await
    }

    pub async fn delete_item(&self, item_id: u64) -> BoxedResult<()> {
        self.client
            .delete::<(), ()>(
                &format!("users/@me/notification-center/items/{}", item_id),
                None::<()>,
                Some(RequestProperties::home()),
            )
            .await
    }
}
