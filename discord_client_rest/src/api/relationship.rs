use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{HomePageReferer, Referer};
use discord_client_structs::structs::user::relationship::Relationship;
use serde_json::Value;

pub struct RelationshipRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> RelationshipRest<'a> {
    fn props(&self) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(HomePageReferer {}.into())
            .build()?)
    }

    pub async fn get_all(&self) -> BoxedResult<Vec<Relationship>> {
        let path = "users/@me/relationships";

        self.client
            .get::<Vec<Relationship>>(&path, None, Some(self.props()?))
            .await
    }

    pub async fn get_game_relationships(&self) -> BoxedResult<Value> {
        let path = "users/@me/game-relationships";

        self.client
            .get::<Value>(&path, None, Some(self.props()?))
            .await
    }

    pub async fn get_friend_suggestions(&self) -> BoxedResult<Value> {
        let path = "friend-suggestions";

        self.client
            .get::<Value>(&path, None, Some(self.props()?))
            .await
    }

    pub async fn ignore(&self, user_id: u64) -> BoxedResult<()> {
        let path = format!("users/@me/relationships/{}/ignore", user_id);

        self.client
            .put::<(), ()>(&path, None::<()>, Some(self.props()?))
            .await
    }

    pub async fn unignore(&self, user_id: u64) -> BoxedResult<()> {
        let path = format!("users/@me/relationships/{}/ignore", user_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(self.props()?))
            .await
    }

    pub async fn remove(&self, user_id: u64) -> BoxedResult<()> {
        let path = format!("users/@me/relationships/{}", user_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(self.props()?))
            .await
    }
}
