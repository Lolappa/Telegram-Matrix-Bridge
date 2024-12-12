use frankenstein::{
    Api,
    GetUpdatesParams,
    api_params::{ ReplyParameters, SendMessageParams },
    objects::UpdateContent,
    TelegramApi,
};
use matrix_sdk::ruma::events::room::message::{
    FormattedBody, MessageFormat, MessageType, RoomMessageEventContent, TextMessageEventContent
};
use std::env;

fn main() {
    // Argument parsing
    let args: Vec<String> = env::args().collect();
    let api_key = args.get(1).expect("Please provide bot token");

    let api = Api::new(api_key);

    let mut update_params = GetUpdatesParams::builder().build();

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {result:?}");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let reply_parameters = ReplyParameters::builder()
                            .message_id(message.message_id)
                            .build();
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(message.text.as_ref().unwrap())
                            .reply_parameters(reply_parameters)
                            .build();
                        if let Err(error) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {error:?}");
                        }

                        let mut content = TextMessageEventContent::plain(message.text.as_ref().unwrap());
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

trait Escape {
    fn escape(&self) -> Self;
    fn unescape(&self) -> Self;
}

impl Escape for String {
    fn escape(&self) -> Self {
        todo!();
    }
    fn unescape(&self) -> Self {
        todo!();
    }
}
