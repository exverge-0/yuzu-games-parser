mod serialize;

use serde::{Deserialize, Serialize};
use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let games: Vec<Game> = serde_json::from_str(include_str!("websiteFeed"))?; // must be in src folder
    let eshop: Value = serde_json::from_str(include_str!("US.en.json"))?; // must be in src folder
    let id = std::env::args().nth(1).unwrap();

    let mut result: Option<Game> = None;

    for game in games {
        if game.id == id {
            result = Some(game);
            break;
        }
    }
    if result.is_none() {
        eprintln!("id not found");
        std::process::exit(1);
    }
    let result = result.unwrap();
    let title_id = result.releases.get(0).unwrap().id.clone();

    let mut eshop_id = String::new();
    for (str, title) in eshop.as_object().unwrap() {
        let id = title.get("id").unwrap().as_str();
        if id.is_none() { continue; }
        if id.unwrap().to_string() == title_id {
            eshop_id = str.clone();
            break;
        }
    }
    if eshop_id.is_empty() {
        eprintln!("eshop id not found");
        std::process::exit(1);
    }

    println!("{}", serde_json::to_string_pretty(&serialize::Game {
        name: result.title.clone(),
        description: eshop.get(&eshop_id).unwrap().get("description").unwrap().as_str().unwrap().to_string(),
        titleId: title_id,
        img: eshop.get(&eshop_id).unwrap().get("iconUrl").unwrap().as_str().unwrap().to_string(),
        tests: result.testcases.iter().map(|x| serialize::testcase_to_test(x)).collect(),
    }).unwrap());
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Game {
    id: String,
    title: String,
    wiki_override: Option<String>,
    // this is only used in certain pokémon games
    wiki_markdown: Option<String>,
    compatibility: i8,
    releases: Vec<Release>,
    testcase_date: Option<String>,
    testcases: Vec<TestCase>,
    issues: Vec<Issue>,
    savefiles: Vec<Savefile>,
}

#[derive(Deserialize, Serialize)]
struct Release {
    id: String,
    region: Option<String>,
    // this is always null
    release_date: Option<String>,
}

#[derive(Deserialize, Serialize)]
enum OS {
    Windows,
    Linux,
}

#[derive(Deserialize, Serialize)]
struct TestCase {
    id: String,
    author: Option<String>,
    #[serde(rename = "buildDate")]
    build_date: String,
    #[serde(rename = "buildName")]
    build_name: String,
    version: String,
    compatibility: i8,
    cpu: String,
    gpu: String,
    date: String,
    os: OS,
    program_id: String,
}

#[derive(Deserialize, Serialize)]
struct Issue {
    created_at: String,
    id: u16,
    // there can't be more than 65,535 yuzu & suyu issues... right?
    owner_username: String,
    state: IssueState,
    tags: Option<String>,
    // either null, array encased in a string, or just a singular string
    title: String,
    updated_at: String,
}

#[derive(Deserialize, Serialize)]
struct Savefile {
    author: String,
    basename: String,
    description: String,
    id: String,
    title: String,
    title_id: String,
}

#[derive(Deserialize, Serialize)]
enum IssueState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
}