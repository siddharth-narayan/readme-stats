use std::{env, fs};

use axum::extract::Path;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::util::GraphQLNodes;

#[derive(Deserialize, Debug)]
struct QueryResponse {
  data: Data
}

#[derive(Deserialize, Debug)]
struct Data {
  user: User,
}

#[derive(Deserialize, Debug)]
struct User {
  repositories: GraphQLNodes<Repo>
}

#[derive(Deserialize, Debug)]
struct Repo {
  name: String,
  languages: Languages
}

#[derive(Deserialize, Debug)]
struct Languages {
  edges: Vec<Language>
}

#[derive(Deserialize, Debug)]
struct Language {
  size: usize,
  node: LangNameNode
}

#[derive(Deserialize, Debug)]
struct LangNameNode {
  name: String
}

pub async fn languages(Path(username): Path<String>) -> () {
    let client = reqwest::ClientBuilder::new().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:154.0) Gecko/20100101 Firefox/154.0").build().unwrap();
    
    let bearer = env::var("GITHUB_TOKEN").unwrap();
    let repo_response = client
      .post("https://api.github.com/graphql")
      .body(json!({
        "query": include_str!("langs.graphql"),
        "variables": {
          "login": username
        }
      }).to_string())
      .bearer_auth(&bearer)
      .send().await.map_err(|_| StatusCode::from_u16(400).unwrap()).unwrap();
    
    // let text = repo_response.text().await.unwrap();
    // fs::write("out", &text);

    // let r: QueryResponse = serde_json::from_str(&text).unwrap();
    println!("{:#?}", repo_response.json::<QueryResponse>().await.unwrap());
}
