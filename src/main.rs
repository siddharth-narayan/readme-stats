use std::{collections::HashMap, env};

use axum::{Router, extract::Path, routing::get};
use reqwest::StatusCode;
use serde::Deserialize;
use typst::{
    Library, LibraryExt, diag::FileResult, foundations::{Bytes, Datetime, Dict, Duration, Str}, syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot}, text::{Font, FontBook}, utils::LazyHash,
};
use typst_kit::{
    datetime::Time, downloader::SystemDownloader, files::{FileStore, FsRoot, SystemFiles}, fonts::{self, FontStore}, packages::SystemPackages,
};
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

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

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new().route("/repos/{user}/{repo}", get(handle_req));
    
    axum::serve(listener, app).await.unwrap();
}

async fn handle_req(Path((username, repo_name)): Path<(String, String)>) -> Result<String, StatusCode> {
    let client = reqwest::ClientBuilder::new().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:154.0) Gecko/20100101 Firefox/154.0").build().unwrap();
    
    let bearer = env::var("GITHUB_TOKEN").unwrap();
    let repo_response = client.get(format!("https://api.github.com/repos/{username}/{repo_name}")).bearer_auth(&bearer).send().await.map_err(|_| StatusCode::from_u16(400).unwrap())?;
    let repo_lang_response = client.get(format!("https://api.github.com/repos/{username}/{repo_name}/languages")).bearer_auth(&bearer).send().await.map_err(|_| StatusCode::from_u16(401).unwrap())?;
    
    let repo = repo_response.json::<Repo>().await.map_err(|e| { println!("{:#?}", e); StatusCode::from_u16(402).unwrap() })?;
    let repo_langs = repo_lang_response.json::<HashMap<String, usize>>().await.map_err(|_| StatusCode::from_u16(403).unwrap())?;

    let mut inputs = Dict::new();
    inputs.insert(Str::from("repo-name"), typst::foundations::Value::Str(Str::from(repo_name)));
    inputs.insert(Str::from("repo-desc"), typst::foundations::Value::Str(Str::from(repo.description.unwrap_or_default())));

    let lang = repo_langs.iter().max_by(|l1, l2| { l1.1.cmp(l2.1)}).map(|f| f.0.clone()).unwrap_or_default();
    inputs.insert(Str::from("repo-lang"), typst::foundations::Value::Str(Str::from(lang)));

    let world = World::new(inputs);
    let result = typst::compile(&world);
    let document: PagedDocument = result.output.unwrap();

    Ok(typst_svg::svg(&document.pages()[0], &SvgOptions::default()))
}

pub struct World {
    lib: LazyHash<Library>,
    fonts: FontStore,
    main: FileId,
    file_store: FileStore<SystemFiles>,
}

impl World {
    pub fn new(inputs: Dict) -> World {
        let file_store = FileStore::new(SystemFiles::new(
            FsRoot::new(".".into()),
            SystemPackages::new(SystemDownloader::new("")),
        ));

        let main_path = VirtualPath::new("test.typ").unwrap();
        let main = FileId::new(RootedPath::new(VirtualRoot::Project, main_path));

        let mut fonts = FontStore::new();
        fonts.extend(fonts::system());
        fonts.extend(fonts::embedded());

        World {
            lib: Library::builder().with_inputs(inputs).build().into(),
            fonts,
            main,
            file_store,
        }
    }
}

impl typst::World for World {
    fn library(&self) -> &LazyHash<Library> {
        &self.lib
    }

    fn book(&self) -> &LazyHash<FontBook> {
        self.fonts.book()
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.file_store.source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file_store.file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        Time::system().today(offset)
    }
}