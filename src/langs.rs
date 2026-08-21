use std::{collections::HashMap, env};

use axum::{extract::{Path, Query}, http::HeaderMap};
use indexmap::IndexMap;
use reqwest::StatusCode;
use rustc_hash::FxBuildHasher;
use serde::Deserialize;
use serde_json::json;
use tokio::task::spawn_blocking;
use typst::{foundations::{Dict, Str, Value}, layout::Ratio};

use crate::{util::{GraphQLNodes, SharedParams, compile_svg}};

#[derive(Deserialize)]
pub struct Params {
    ignore_repos: Option<String>,
    ignore_langs: Option<String>,

    #[serde(flatten)]
    shared: SharedParams
}

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

fn sort_langs(query_resp: QueryResponse, ignore_repos: Option<String>, ignore_langs: Option<String>) -> Dict {

  let ignore_repos = match ignore_repos {
    Some(s) => s.split(',').map(String::from).collect(),
    None => Vec::new()
  };

  let ignore_langs = match ignore_langs {
    Some(s) => s.clone().split(',').map(String::from).collect(),
    None => Vec::new()
  };

  let mut lang_bytecount = HashMap::new();
  let mut total_bytes = 0;
  for repo in query_resp.data.user.repositories.nodes {
    if ignore_repos.contains(&repo.name) {
      continue;
    }

    for language in repo.languages.edges {
      let name = language.node.name;
      if ignore_langs.contains(&name) {
        continue
      }

      let size = language.size;

      total_bytes += size;

      match lang_bytecount.get_mut(&name) {
        Some(cur_size) => {
          *cur_size += size;
        },
        None => {
          lang_bytecount.insert(name, size);
        }
      }
    }
  }

  let mut out: Vec<(String, f64)> = lang_bytecount.into_iter().map(|(name, count)| (name, count as f64 / total_bytes as f64)).collect();
  out.sort_by(|l1, l2| l2.1.partial_cmp(&l1.1).unwrap_or(std::cmp::Ordering::Equal));
  out.truncate(6);
  
  let map: IndexMap<Str, Value, FxBuildHasher> = IndexMap::from_iter(out.into_iter().map(|(name, frac)| (Str::from(name), Value::Ratio(Ratio::new(frac)))));

  Dict::from(map)

}

pub async fn languages(Path(username): Path<String>, Query(lang_params): Query<Params>) -> Result<(HeaderMap, String), StatusCode> {
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
      .send().await.map_err(|_| StatusCode::from_u16(418).unwrap()).unwrap();

      let response = repo_response.json::<QueryResponse>().await.unwrap();
      let langs = sort_langs(response, lang_params.ignore_repos, lang_params.ignore_langs);

      let mut inputs = Dict::new();
      inputs.insert(Str::from("languages"), Value::Dict(langs));
      inputs.insert(Str::from("theme"), Value::Str(Str::from(lang_params.shared.theme.unwrap_or_default().to_str())));

      let svg_text = spawn_blocking(|| compile_svg("tiles/langs.typ", inputs)).await.unwrap().map_err(|_| StatusCode::BAD_REQUEST)?;

      let mut headers = HeaderMap::new();
      headers.insert("content-type", "image/svg+xml".parse().unwrap());

      Ok((headers, svg_text))
}
