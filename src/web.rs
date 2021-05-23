use crate::command::BunnyCommand;
use actix_web::{get, http, web, HttpResponse, Responder};
use serde::Deserialize;
use std::collections::HashMap;

pub struct AppState<'a> {
    pub command_table: HashMap<String, BunnyCommand<'a>>,
    pub default_command: BunnyCommand<'a>,
}

#[derive(Deserialize)]
pub struct Search {
    q: Option<String>,
}

#[get("/search")]
pub async fn search(
    search_data: web::Query<Search>,
    data: web::Data<AppState<'_>>,
) -> impl Responder {
    if let Some(query) = &search_data.q {
        let args = query
            .split(" ")
            .map(|value| value.to_string())
            .collect::<Vec<_>>();
        let tag = args.get(0).map(|value| value.clone());

        let url = tag
            .and_then(|tag| {
                data.command_table
                    .get(&tag)
                    .and_then(|command| (*command.action)(&args[1..]))
            })
            .unwrap_or_else(|| {
                (*data.default_command.action)(&args).unwrap_or("https://google.com".to_string())
            });

        HttpResponse::Found()
            .header(http::header::LOCATION, url)
            .finish()
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, "https://google.com")
            .finish()
    }
}
