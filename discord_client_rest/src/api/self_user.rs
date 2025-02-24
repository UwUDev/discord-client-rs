use crate::BoxedResult;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{HomePageReferer, Referer};
use chrono::{DateTime, Utc};
use discord_client_structs::structs::user::User;
use serde_json::{Value, json};

pub struct SelfUserRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> SelfUserRest<'a> {
    pub async fn get(&self) -> BoxedResult<User> {
        let path = String::from("users/@me");

        let referer = HomePageReferer {};

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client.get::<User>(&path, None, Some(props)).await
    }

    async fn patch(&self, body: Value) -> BoxedResult<User> {
        let path = String::from("users/@me");

        let referer = HomePageReferer {};

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .patch::<User, Value>(&path, Some(body), Some(props))
            .await
    }

    pub async fn change_username(&self, username: String) -> BoxedResult<User> {
        let body = json!({
            "username": username,
        });

        self.patch(body).await
    }

    pub async fn change_global_name(&self, name: String) -> BoxedResult<User> {
        let body = json!({
            "global_name": name,
        });

        self.patch(body).await
    }

    pub async fn change_email(&self, email: String, email_token: String) -> BoxedResult<User> {
        let body = json!({
            "email": email,
            "email_token": email_token,
        });

        self.patch(body).await
    }

    pub async fn change_pronouns(&self, pronouns: String) -> BoxedResult<User> {
        let body = json!({
            "pronouns": pronouns,
        });

        self.patch(body).await
    }

    pub async fn change_bio(&self, bio: String) -> BoxedResult<User> {
        let body = json!({
            "bio": bio,
        });

        self.patch(body).await
    }

    pub async fn change_accent_color(&self, accent_color: u64) -> BoxedResult<User> {
        let body = json!({
            "accent_color": accent_color,
        });

        self.patch(body).await
    }

    pub async fn change_date_of_birth(&self, date_of_birth: DateTime<Utc>) -> BoxedResult<User> {
        let body = json!({
            "date_of_birth": date_of_birth.to_rfc3339(),
        });

        self.patch(body).await
    }

    pub async fn change_password(
        &self,
        current_password: String,
        new_password: String,
    ) -> BoxedResult<User> {
        let body = json!({
            "password": current_password,
            "new_password": new_password,
        });

        self.patch(body).await
    }

    // todo: avatar, banner, decoration & flags
}
