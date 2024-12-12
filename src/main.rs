use frankenstein::{
    Api,
    GetUpdatesParams,
    api_params::{ ReplyParameters, SendMessageParams },
    objects::UpdateContent,
    TelegramApi,
};
use matrix_sdk::{ruma::events::room::message::{
    FormattedBody, MessageFormat, MessageType, RoomMessageEventContent, TextMessageEventContent
}, Client};
use std::env;

fn main() {
    // Argument parsing
    let (bot_token, homeserver_url, username, password) =
        match (env::args().nth(1), env::args().nth(2), env::args().nth(3), env::args().nth(4)) {
            (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
            _ => {
                eprintln!(
                    "Usage: {} <telegram bot token> <homeserver_url> <username> <password>",
                    env::args().next().unwrap()
                );
            return;
            }
        };
    main_loop(bot_token, homeserver_url, username, password);
}

async fn main_loop(
    bot_token: String,
    homeserver_url: String,
    username: String,
    password: String
) {
    let api = Api::new(&bot_token);
    let matrix_client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await
        .expect("Failed to connect to Matrix");

    matrix_client
        .matrix_auth()
        .login_username(&username, &password)
        .initial_device_display_name("Telegram bridge")
        .await
        .expect("Failed to log in to Matrix");

    println!("logged in Matrix as {}", username);

    let mut update_params = GetUpdatesParams::builder().build();

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {result:?}");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        /*let reply_parameters = ReplyParameters::builder()
                            .message_id(message.message_id)
                            .build();
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(message.text.as_ref().unwrap())
                            .reply_parameters(reply_parameters)
                            .build();
                        if let Err(error) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {error:?}");
                        }*/

                        let mut content = TextMessageEventContent::plain(
                            message.text.as_ref().unwrap()
                        );
                        content.formatted = Some(FormattedBody {
                            format: MessageFormat::from("bridge"),
                            body: message.text.as_ref().unwrap().to_string()
                        });
                        let content = RoomMessageEventContent::new(
                            MessageType::Text(content)
                        );
                    }
                    update_params.offset = Some(i64::from(update.update_id) + 1);
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }
}
