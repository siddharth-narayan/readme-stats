use axum::{Router, routing::get};
use typst::{
    Library, LibraryExt, diag::FileResult, foundations::{Bytes, Datetime, Dict, Duration, Str}, syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot}, text::{Font, FontBook}, utils::LazyHash,
};
use typst_kit::{
    datetime::Time, downloader::SystemDownloader, files::{FileStore, FsRoot, SystemFiles}, fonts::{self, FontStore}, packages::SystemPackages,
};
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_req));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let world = World::new();

    let result = typst::compile(&world);
    let document: PagedDocument = result.output.unwrap();

    let out = typst_svg::svg(
        &document.pages()[0],
        &SvgOptions {
            render_bleed: false,
            pretty: true,
        },
    );
    println!("{}", out)
}

async fn handle_req() -> String {
    "Hello".into()
}

pub struct World {
    lib: LazyHash<Library>,
    fonts: FontStore,
    main: FileId,
    file_store: FileStore<SystemFiles>,
}

impl World {
    pub fn new() -> World {
        let file_store = FileStore::new(SystemFiles::new(
            FsRoot::new(".".into()),
            SystemPackages::new(SystemDownloader::new("")),
        ));

        let main_path = VirtualPath::new("test.typ").unwrap();
        let main = FileId::new(RootedPath::new(VirtualRoot::Project, main_path));

        let mut fonts = FontStore::new();
        fonts.extend(fonts::system());
        fonts.extend(fonts::embedded());

        let mut inputs = Dict::new();
        inputs.insert(Str::from("repo-name"), typst::foundations::Value::Str(Str::from("readme-stats")));
        inputs.insert(Str::from("repo-desc"), typst::foundations::Value::Str(Str::from("A GitHub card generator")));
        inputs.insert(Str::from("repo-lang"), typst::foundations::Value::Str(Str::from("Rust")));

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