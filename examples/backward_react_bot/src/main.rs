use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::client::ClientSession;
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::message::query::MessageSearchQueryBuilder;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let client_session = ClientSession::new();

    let client = RestClient::connect(token.clone(), None, None, Some(client_session))
        .await
        .unwrap();

    let mut before = 1514103735796895764u64;
    let guild_id = 1442887417202868366u64;
    loop {
        let search_query = MessageSearchQueryBuilder::default()
            .author_id(vec![464412791437131776u64])
            .max_id(before - 1)
            .build()
            .unwrap();

        let search_result = client
            .guild(Some(guild_id))
            .search_guild_messages(search_query)
            .await
            .unwrap();

        let messages: Vec<Message> = search_result
            .messages
            .iter()
            .flat_map(|inner_vec| inner_vec.iter())
            .cloned()
            .collect();

        if messages.is_empty() {
            println!("No messages found");
            exit(0);
        }

        for message in messages {
            let url = format!(
                "https://discord.com/channels/{guild_id}/{}/{}",
                message.channel_id, message.id
            );

            match client
                .message(message.channel_id)
                .add_reaction(
                    message.id,
                    "pregnant:1483054477031506011".parse().unwrap(),
                    false,
                    message.guild_id,
                )
                .await
            {
                Ok(_) => {
                    println!("Reacted: {url}");
                }
                Err(err) => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open("fails.txt")
                        .unwrap();

                    if let Err(e) = writeln!(file, "{url}\n") {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    eprintln!("{}", err)
                }
            }
            before = message.id;
        }
    }
}
