use futures::prelude::*;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TCPTransport;
use twitch_irc::TwitchIRCClient;

#[tokio::main]
pub async fn main() {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.next().await {
            println!("Received message: {:?}", message);
        }
    });

    client.join("simon36".to_owned());

    join_handle.await.unwrap();
}
