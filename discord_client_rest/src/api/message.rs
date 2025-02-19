use crate::BoxedResult;
use crate::rest::RestClient;
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::message::query::MessageQuery;

pub struct MessageRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> MessageRest<'a> {
    pub async fn send(&self, channel_id: u64, message: Message) -> BoxedResult<Message> {
        let path = format!("channels/{}/messages", channel_id);
        self.client
            .post::<Message, Message>(&path, Some(message))
            .await
    }

    pub async fn edit(
        &self,
        channel_id: u64,
        message_id: u64,
        message: Message,
    ) -> BoxedResult<Message> {
        let path = format!("channels/{}/messages/{}", channel_id, message_id);
        self.client
            .patch::<Message, Message>(&path, Some(message))
            .await
    }

    pub async fn delete(&self, channel_id: u64, message_id: u64) -> BoxedResult<()> {
        let path = format!("channels/{}/messages/{}", channel_id, message_id);
        self.client.delete::<_, ()>(&path, None::<()>).await
    }

    pub async fn get_channel_messages(
        &self,
        channel_id: u64,
        query: MessageQuery,
    ) -> BoxedResult<Vec<Message>> {
        let path = format!("channels/{}/messages", channel_id);
        self.client
            .get::<Vec<Message>>(&path, Some(query.to_map()))
            .await
    }
}
