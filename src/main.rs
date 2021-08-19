use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use twitch_irc::message::{IRCMessage, ServerMessage};


#[tokio::main]
pub async fn main() {
  let mut channel_name = "fukumituber";
  let config = ClientConfig::default();
  let (mut incoming_messages, client) = 
    TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

  let join_handle = tokio::spawn(async move {
    while let Some(message) = incoming_messages.recv().await {
      match message {
        ServerMessage::Privmsg(msg) => {
          println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);
        },
        ServerMessage::Whisper(msg) => {
          println!("(w) {}: {}", msg.sender.name, msg.message_text);
        },
        _ => {}
      }
    }
  });
  
  client.join(channel_name.to_owned());

  join_handle.await.unwrap();
}
