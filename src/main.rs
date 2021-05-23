use actix_web::{App, HttpServer};
use command::generate_bunny_table;
use urlencoding::encode;
use web::{search, AppState};

use crate::command::BunnyCommand;

mod command;
mod web;

const PORT: i32 = 8090;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let command_table = generate_bunny_table();
        App::new()
            .data(AppState {
                command_table,
                default_command: BunnyCommand {
                    matchers: vec![],
                    action: &|args| {
                        Some(format!(
                            "https://www.google.com/search?q={}",
                            encode(&args.join(" "))
                        ))
                    },
                },
            })
            .service(search)
    })
    // .workers(1)
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
