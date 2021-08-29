extern crate dotenv;

use dotenv::dotenv;
use std::env;

use std::convert::TryInto;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use twitch_irc::message::{IRCMessage, ServerMessage};
use chrono::{Datelike, Timelike, Utc};
use std::sync::Arc;
use std::borrow::Cow;

//TODO use std::in for entering botname and multiple channels, then parse multiple channel joining, using substrings and do a for loop on all of the channel names

#[tokio::main]
pub async fn main() {
  dotenv().ok();
  //let now = Utc::now();
  //let(is_pm, hour) = now.hour12();
  // let bot_name_str = Arc::new(String::from("zowlbot").to_lowercase()); //Change to reading from a file or UI Input
  //Arc vs Cow? apparently Arc is expensive, so trying to use cow for now.
  let bot_name_str = Cow::from(String::from("zowlbot").to_lowercase()); //Change to reading from a file or UI Input
  // let bot_name = &bot_name_str;
  let oauth_token_str = env::var("TWITCH_OAUTH").unwrap().to_string();
  // let channel_name_str = Arc::new(String::from("devizowl").to_lowercase());
  let mut channels: Vec<String> = vec![String::new(); 1];
  channels[0] = "devizowl".to_string();
  // channels[1] = "zowlbot".to_string();
  channels.push("zowlbot".to_string());
  // let channel_name = channel_name_str;
  // let bot_name = bot_name_str;
  // println!("S:{}, W:{}", Arc::strong_count(&bot_name_str), Arc::weak_count(&bot_name_str));
  // let config = ClientConfig::new_simple(StaticLoginCredentials::new(Arc::clone(&bot_name_str).to_string(), Some(oauth_token_str)));
  let config = ClientConfig::new_simple(StaticLoginCredentials::new(bot_name_str.to_string(), Some(oauth_token_str)));
  // let config = ClientConfig::default();
  
  let (mut incoming_messages, client) = 
    TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config); 
  
  for channel in &channels {
    client.join(format!("{}", channel));  
  }
  //client.join(format!("{}", channel_name_str));
  
  let join_handle = tokio::spawn(async move {
    while let Some(message) = incoming_messages.recv().await {
      match message {
        ServerMessage::Privmsg(msg) => {
          let now = Utc::now();
          let(_is_pm, hour) = now.hour12();
          println!("[{:02}:{:02}](#{}) {}: {}", hour, now.minute(), msg.channel_login, msg.sender.name, msg.message_text);
          let text = msg.message_text;
          let sender = msg.sender.name;
          let reply = format!("Are you scared >>>{}<<<? :)", sender.to_string());
          // let bot_name = login_name.to_lowercase().to_string();
          // let bot_name = &bot_name_str;
          // println!("botname: {}", bot_name);
          //enable whispering to bot for replies
          if !sender.to_lowercase().eq(&bot_name_str.to_string()) {
            // let chat_message = text.clone();

            //1. Check if bot was @ed 
            //2. Check what command
            //3. Parse value
            //4. Do function

            //Check if bot is called
            if text.to_lowercase().contains(&bot_name_str.to_string())
            {
              println!("{} called", &bot_name_str.to_string());
              //Get exclamation mark and command, then value
              let mut command = "".to_string();
              let mut value = "".to_string();
              text.split_whitespace();
              for s in text.split_whitespace() {
                match s
                {
                  _ if s.to_lowercase().contains(&bot_name_str.to_string()) => {println!("{} found", s)}
                  _ if command != "" => {
                    // println!("value: {}", s); 
                    if value != "" { 
                      value.push_str(" ");
                    } 
                    value.push_str(s);
                  }
                  _ if s.contains("!") => {/*println!("[{}] Command", s); */ command = s.to_string()},
                  _ => println!("No cmd found ({})", s),
                  // &bot_name_str.to_string() => println!("{} found ", &bot_name_str.to_string()),
                  // _ => println!(""),
                }
              }
              println!("FullCMD: {} {}", command, value);
              match command.as_str() {
                "!jpd" | "!JPD" => {
                  println!("Do dictionary search with: {}", value);
                  let entry = jmdict::entries().find(|e| {
                    e.kanji_elements().any(|k| k.text == value)
                  });
                  //TODO: try and get multiple glosses
                  //should maybe error handle the entry unwrapping
                  let reading_form = match entry {
                    Some(ent) => format!("{}「{}」- {} [for {}]", value, ent.reading_elements().next().unwrap().text, ent.senses().next().unwrap().glosses().next().unwrap().text, sender),
                    None => format!("Cannot find「{}」[for {}]", value, sender),
                  };
                  println!("[{:02}:{:02}]{}", hour, now.minute(), reading_form);
                  client.send_message(twitch_irc::message::IRCMessage::new_simple("PRIVMSG".to_string(), vec![format!("#{}", msg.channel_login), reading_form.to_string()])).await.unwrap();
                }
                _ => {println!("Invalid Command"); }
              }
              //Do command

            }

  // println!("S:{}, W:{}", Arc::strong_count(&bot_name_str), Arc::weak_count(&bot_name_str));
            if text.contains("Kappa")  {
              println!("Kappa detected");
            // client.send_message("Kappa Detected")
            }
            if text.contains("monkaS")  {
              println!("monkaS detected");
              println!("{} @{}?", reply, sender);
              client.send_message(twitch_irc::message::IRCMessage::new_simple("PRIVMSG".to_string(), vec![format!("#{}", msg.channel_login), reply])).await.unwrap();
            }
          }
       },
        
        ServerMessage::Whisper(msg) => {
          let now = Utc::now();
          let(_is_pm, hour) = now.hour12();
          println!("[{:02}:{:02}](w) {}: {}", hour, now.minute(), msg.sender.name, msg.message_text);
        },
        _ => {}
      }
    }
  });

  join_handle.await.unwrap();
}
