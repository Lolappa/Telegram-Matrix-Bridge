use frankenstein::{Api, GetUpdatesParams};
use frankenstein::api_params::{ReplyParameters, SendMessageParams};
use frankenstein::objects::UpdateContent;
use frankenstein::TelegramApi;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let api = Api::new(args.get(1).expect("Please provide bot token"));
    let fifo_path = args.get(2).expect("Please provide fifo path");

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
                        let mut file = OpenOptions::new().write(true).open(fifo_path).expect("Failed to open file");
                        file.write(message.text.as_ref().unwrap().as_bytes());
                        //file.sync_all();
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
