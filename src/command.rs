use std::collections::HashMap;
use urlencoding::encode;

#[derive(Clone)]
pub struct BunnyCommand<'a> {
    pub matchers: Vec<String>,
    pub action: &'a dyn Fn(&[String]) -> Option<String>,
}

pub fn generate_bunny_table<'a>() -> HashMap<String, BunnyCommand<'a>> {
    let commands = vec![
        BunnyCommand {
            matchers: vec!["y".to_string(), "yt".to_string()],
            action: &|args| {
                if args.len() > 0 {
                    Some(format!(
                        "https://www.youtube.com/results?search_query={}",
                        encode(&args.join(" "))
                    ))
                } else {
                    Some("https://youtube.com".to_string())
                }
            },
        },
        BunnyCommand {
            matchers: vec!["g".to_string(), "gm".to_string()],
            action: &|_| Some("https://gmail.com".to_string()),
        },
        BunnyCommand {
            matchers: vec!["c".to_string(), "cal".to_string()],
            action: &|_| Some("https://calendar.google.com".to_string()),
        },
        BunnyCommand {
            matchers: vec!["gd".to_string()],
            action: &|args| match args.get(0) {
                Some(location) if location == "p" || location == "personal" => {
                    Some("https://drive.google.com/drive/u/0/my-drive".to_string())
                }
                Some(location) if location == "s" || location == "school" => {
                    Some("https://drive.google.com/drive/u/1/my-drive".to_string())
                }
                _ => Some("https://drive.google.com/drive".to_string()),
            },
        },
        BunnyCommand {
            matchers: vec!["gh".to_string()],
            action: &|args| match args.get(0) {
                Some(location) if location == "m" || location == "me" => {
                    Some("https://github.com/kpfromer".to_string())
                }
                _ => Some("https://github.com".to_string()),
            },
        },
        BunnyCommand {
            matchers: vec!["n".to_string()],
            action: &|_| Some("https://netflix.com".to_string()),
        },
        BunnyCommand {
            matchers: vec!["r".to_string()],
            action: &|_| Some("https://reddit.com".to_string()),
        },
        BunnyCommand {
            matchers: vec!["pi".to_string()],
            action: &|_| Some("https://netflix.com".to_string()),
        },
        BunnyCommand {
            matchers: vec!["cu".to_string()],
            action: &|_| Some("https://buffportal.colorado.edu/".to_string()),
        },
        BunnyCommand {
            matchers: vec!["c".to_string(), "canvas".to_string()],
            action: &|_| Some("https://canvas.colorado.edu/".to_string()),
        },
        BunnyCommand {
            matchers: vec!["trail".to_string()],
            action: &|_| Some("https://maps.bouldercolorado.gov/osmp-trails/".to_string()),
        },
        // TODO: remove
        BunnyCommand {
            matchers: vec!["test".to_string()],
            action: &|_| Some("https://google.com".to_string()),
        },
    ];

    let table = commands
        .iter()
        .flat_map(|command| {
            command
                .matchers
                .clone()
                .into_iter()
                .map(|matcher| (matcher, command.clone()))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<String, BunnyCommand<'a>>>();

    table
}
