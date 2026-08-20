use std::{env, time::{SystemTime, UNIX_EPOCH}};

use axum::{extract::{Path, Query}, http::HeaderMap};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use typst::{foundations::{Dict, Str, Value}};
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

use crate::{util::{GraphQLNodes, SharedParams}, world::World};

#[derive(Deserialize, Debug)]
struct QueryResponse {
  data: Data
}

#[derive(Deserialize, Debug)]
struct Data {
  user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
  name: Option<String>, // Why in the world can it be null
  repositories: GraphQLNodes<Repo>,
  commits: Commits,
  pull_requests: PullRequests,
  issues: Issues,
  repositories_contributed_to: ContributedTo
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Repo {
  stargazer_count: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Commits {
  total_count: usize
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PullRequests {
  total_count: usize
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Issues {
  total_count: usize
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContributedTo {
  total_count: usize
}


#[derive(Deserialize)]
pub struct Params {
    #[serde(flatten)]
    shared: SharedParams
}

struct Stats {
  name: String,
  stars: usize,
  commits: usize,
  pull_requests: usize,
  issues: usize,
  contributions: usize
}

impl Stats {
  pub fn new(login: String, response: QueryResponse) -> Stats {
    let user = response.data.user;

    Stats {
      name: user.name.unwrap_or(login),
      stars: user.repositories.nodes.iter().map(|r| r.stargazer_count).sum(),
      commits: user.commits.total_count,
      pull_requests: user.pull_requests.total_count,
      issues: user.issues.total_count,
      contributions: user.repositories_contributed_to.total_count
    }
  }
}

pub async fn stats(Path(username): Path<String>, Query(lang_params): Query<Params>) -> Result<(HeaderMap, String), StatusCode> {
    let client = reqwest::ClientBuilder::new().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:154.0) Gecko/20100101 Firefox/154.0").build().unwrap();
    let bearer = env::var("GITHUB_TOKEN").unwrap();

    let from = DateTime::<Utc>::from(UNIX_EPOCH).to_rfc3339();
    let to = DateTime::<Utc>::from(SystemTime::now()).to_rfc3339();

    let repo_response = client
      .post("https://api.github.com/graphql")
      .body(json!({
        "query": include_str!("stats.graphql"),
        "variables": {
          "login": username,
        }
      }).to_string())
      .bearer_auth(&bearer)
      .send().await.map_err(|_| StatusCode::from_u16(418).unwrap()).unwrap();

      let response = repo_response.json::<QueryResponse>().await.unwrap();
      let stats = Stats::new(username, response);

      let mut inputs = Dict::new();

      inputs.insert(Str::from("name"), Value::Str(stats.name.into()));
      inputs.insert(Str::from("star-count"), Value::Int(stats.stars as i64));
      inputs.insert(Str::from("commits"), Value::Int(stats.commits as i64));
      inputs.insert(Str::from("pull-requests"), Value::Int(stats.pull_requests as i64));
      inputs.insert(Str::from("issues"), Value::Int(stats.issues as i64));
      inputs.insert(Str::from("repo-contributions"), Value::Int(stats.contributions as i64));

      inputs.insert(Str::from("theme"), Value::Str(Str::from(lang_params.shared.theme.unwrap_or_default().to_str())));

      let world = World::new("tiles/stats.typ", inputs);

      // This unwrap needs to go
      let document: PagedDocument = typst::compile(&world).output.unwrap();

      let svg_text = typst_svg::svg(&document.pages()[0], &SvgOptions::default());

      let mut headers = HeaderMap::new();
      headers.insert("content-type", "image/svg+xml".parse().unwrap());

      // Ok((HeaderMap::new(), String::new()))
      Ok((headers, svg_text))
}
