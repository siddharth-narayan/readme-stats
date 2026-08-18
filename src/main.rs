use std::io::{Error, ErrorKind};

use typst::{
    Library, LibraryExt,
    diag::FileResult,
    foundations::{Bytes, Datetime, Duration},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::{
    files::{FileStore, FsRoot, SystemFiles},
    fonts::FontStore,
    packages::SystemPackages,
};
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

fn main() {
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
            SystemPackages::new(Downloader),
        ));

        let main_path = VirtualPath::new("test.typ").unwrap();
        let main = FileId::new(RootedPath::new(VirtualRoot::Project, main_path));

        let mut fonts = FontStore::new();
        fonts.extend(typst_kit::fonts::system());

        World {
            lib: Library::builder().build().into(),
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
        todo!()
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        Datetime::from_ymd(1970, 1, 1)
    }
}

struct Downloader;
impl typst_kit::downloader::Downloader for Downloader {
    fn stream(
        &self,
        key: &dyn std::any::Any,
        url: &str,
    ) -> std::io::Result<(Option<usize>, Box<dyn std::io::prelude::Read>)> {
        std::io::Result::Err(Error::from(ErrorKind::AddrInUse))
    }
}
