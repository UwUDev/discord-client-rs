use discord_client_gateway::gateway::GatewayClient;
use discord_client_rest::captcha::{CaptchaRequiredError, SolvedCaptcha};
use discord_client_rest::rest::RestClient;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let rest_client = RestClient::connect(token.clone(), None, None)
        .await
        .unwrap();

    let mut gateway_client = GatewayClient::connect(token, 53607934, rest_client.build_number)
        .await
        .unwrap();

    let user_id = rest_client.user_id;

    println!("User ID: {}", user_id);

    for _ in 0..2 {
        let _ = gateway_client.next_event().await.unwrap();
    }

    let session_id = gateway_client.session_id.clone().unwrap();

    tokio::spawn(async move {
        loop {
            let _ = gateway_client.next_event().await.unwrap();
        }
    });

    //ask console for invite code
    let mut invite_code = String::new();
    println!("Enter invite code:");
    std::io::stdin().read_line(&mut invite_code).unwrap();
    invite_code = invite_code.trim().to_string();

    //get invite
    let invite = rest_client.invite().get_invite(invite_code).await.unwrap();
    println!(
        "Invite channel name: {}",
        invite.channel.clone().unwrap().name.unwrap()
    );

    //join invite
    match rest_client
        .invite()
        .join_invite(invite.clone(), session_id.clone(), None)
        .await
    {
        Ok(_) => {
            println!("Joined invite");
        }
        Err(err) => {
            if let Some(captcha) = err.downcast_ref::<CaptchaRequiredError>() {
                println!("Captcha required");
                println!("- Sitekey: {}", captcha.captcha_sitekey);
                println!("- URL: https://discord.com/channels/@me");
                println!("- RQ Data: {}", captcha.captcha_rqdata);
                let rqtoken = captcha.captcha_rqtoken.clone();

                let mut key = String::new();
                println!("Enter captcha key:");
                std::io::stdin().read_line(&mut key).unwrap();
                key = key.trim().to_string();

                let solved_captcha = SolvedCaptcha::new(key, rqtoken);

                match rest_client
                    .invite()
                    .join_invite(invite, session_id, Some(solved_captcha))
                    .await
                {
                    Ok(_) => {
                        println!("Joined invite");
                    }
                    Err(err) => {
                        println!("Error joining invite: {:?}", err);
                    }
                }
            }
        }
    }
}
