extern crate serenity;

use std::env;

use serenity::client::{Client, Context};
use serenity::framework::standard::{Args, Command, CommandError, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::prelude::EventHandler;
use serenity::Error;

struct Handler;

impl EventHandler for Handler {}

pub fn run() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Need a token"), Handler)
        .expect("Error when creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .cmd("point", PointCommand::new()),
    );

    if let Err(why) = client.start() {
        eprintln!("An error occured: {:?}", why);
    }
}

struct Point {
    pub keys: &'static [&'static str],
    pub description: &'static str,
    pub link: &'static str,
}

struct PointCommand {
    points: &'static [Point],
}

impl PointCommand {
    pub fn new() -> Self {
        Self::default()
    }

    fn send_help(&self, user: &User) -> Result<(), Error> {
        let mut s = String::from("Here are the available points:\n");
        for point in self.points {
            s.push_str(&point.keys.join(", "));
            s.push_str(&format!(" : {}\n", point.description));
        }
        user.direct_message(|m| m.content(s)).map(|_| ())
    }
}

impl Default for PointCommand {
    fn default() -> Self {
        Self {
            points: &[
                Point {
                    keys: &["godwin"],
                    description: "Pour avoir habilement amené la discussion jusqu'à la comparaison finale.",
                    link: "https://scontent-lhr3-1.xx.fbcdn.net/v/t31.0-8/13909010_1325652514129299_6974569039548044434_o.jpg?_nc_cat=0&oh=af1809ecc2c0a19f8dfd2dc0c6fffc2d&oe=5B7E4F18"
                },
                Point {
                    keys: &["new", "nouveaute", "nouveauté"],
                    description: "Pour avoir préféré quelque chose sur la base de sa modernité.",
                    link: "https://scontent-cdg2-1.xx.fbcdn.net/v/t31.0-8/13925671_1326151820746035_7769300352044232363_o.jpg?_nc_cat=0&oh=6392caff456dd85abb2082534464c9ff&oe=5B7F0F3C"
                },
            ]
        }
    }
}

impl Command for PointCommand {
    fn execute(
        &self,
        _context: &mut Context,
        message: &Message,
        mut args: Args,
    ) -> Result<(), CommandError> {
        if let Ok(command) = args.single::<String>() {
            for point in self.points {
                if point.keys.contains(&&*command) {
                    message
                        .channel_id
                        .send_message(|m| m.embed(|e| e.image(point.link)))?;
                    return Ok(());
                }
            }
            self.send_help(&message.author)?;
        } else {
            message.react("🖕")?;
        }
        Ok(())
    }
}
