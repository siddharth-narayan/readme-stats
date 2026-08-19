use std::{collections::HashMap, env};

use axum::{Router, extract::{Path, Query}, routing::get};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use typst::foundations::{Dict, Str, Value};
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

use crate::world::World;

mod world;

#[derive(Deserialize)]
struct GithubUser {
    login: String
}

#[derive(Deserialize)]
struct Repo {
    owner: GithubUser,
    description: Option<String>,
    stargazers_count: usize
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
enum Theme {
    GithubDark,
    #[default]
    GithubLight
}

impl Theme {
    pub fn to_str(self) -> &'static str {
        match self {
            Theme::GithubLight => "github-light",
            Theme::GithubDark => "github-dark"
        }
    }
}

#[derive(Deserialize)]
struct RepoParams {
    theme: Option<Theme>
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new().route("/repos/{user}/{repo}", get(repo_req));
    
    axum::serve(listener, app).await.unwrap();
}

async fn repo_req(Path((username, repo_name)): Path<(String, String)>, Query(params): Query<RepoParams>) -> Result<String, StatusCode> {
    let client = reqwest::ClientBuilder::new().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:154.0) Gecko/20100101 Firefox/154.0").build().unwrap();
    
    let bearer = env::var("GITHUB_TOKEN").unwrap();
    let repo_response = client.get(format!("https://api.github.com/repos/{username}/{repo_name}")).bearer_auth(&bearer).send().await.map_err(|_| StatusCode::from_u16(400).unwrap())?;
    let repo_lang_response = client.get(format!("https://api.github.com/repos/{username}/{repo_name}/languages")).bearer_auth(&bearer).send().await.map_err(|_| StatusCode::from_u16(401).unwrap())?;
    
    let repo = repo_response.json::<Repo>().await.map_err(|e| { println!("{:#?}", e); StatusCode::from_u16(402).unwrap() })?;
    let repo_langs = repo_lang_response.json::<HashMap<String, usize>>().await.map_err(|_| StatusCode::from_u16(403).unwrap())?;
    let repo_toplang = repo_langs.iter().max_by(|l1, l2| { l1.1.cmp(l2.1)}).map(|f| f.0.clone()).unwrap_or_default();

    let mut inputs = Dict::new();
    inputs.insert(Str::from("repo-name"), Value::Str(Str::from(repo_name)));
    inputs.insert(Str::from("repo-desc"), Value::Str(Str::from(repo.description.unwrap_or_default())));
    inputs.insert(Str::from("repo-stars"), Value::Int(repo.stargazers_count.try_into().unwrap()));
    inputs.insert(Str::from("repo-lang"), Value::Str(Str::from(repo_toplang)));
    inputs.insert(Str::from("theme"), Value::Str(Str::from(params.theme.unwrap_or_default().to_str())));

    let world = World::new("repo.typ", inputs);

    // This unwrap needs to go
    let document: PagedDocument = typst::compile(&world).output.unwrap();

    Ok(typst_svg::svg(&document.pages()[0], &SvgOptions::default()))
}