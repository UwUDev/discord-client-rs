use crate::BoxedResult;
use crate::rest::RequestPropertiesBuilder;
use crate::sessionless::SessionlessClient;
use crate::structs::referer::{HomePageReferer, Referer};
use discord_client_structs::structs::user::experiment::ExperimentAssignments;
use std::collections::HashMap;

pub struct SessionlessExperimentsRest<'a> {
    pub client: &'a SessionlessClient,
}

impl<'a> SessionlessExperimentsRest<'a> {
    pub async fn get_assignments(
        &self,
        with_guild_experiments: bool,
    ) -> BoxedResult<ExperimentAssignments> {
        let path = "experiments";

        let mut params = HashMap::new();
        params.insert(
            "with_guild_experiments".to_string(),
            with_guild_experiments.to_string(),
        );

        let referer = HomePageReferer {};
        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        let assignments = self
            .client
            .get::<ExperimentAssignments>(path, Some(params), Some(props))
            .await?;

        if let Some(fingerprint) = &assignments.fingerprint {
            self.client.set_fingerprint(Some(fingerprint.clone())).await;
        }

        Ok(assignments)
    }

    pub async fn create_fingerprint(&self) -> BoxedResult<String> {
        let path = "auth/fingerprint";

        let referer = HomePageReferer {};
        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        #[derive(serde::Deserialize, Default)]
        struct FingerprintResponse {
            fingerprint: String,
        }

        let resp = self
            .client
            .post::<FingerprintResponse, ()>(path, None, Some(props))
            .await?;

        self.client
            .set_fingerprint(Some(resp.fingerprint.clone()))
            .await;

        Ok(resp.fingerprint)
    }
}
