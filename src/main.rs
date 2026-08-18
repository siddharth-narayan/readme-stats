use std::io::{Error, ErrorKind};

use typst::{Library, LibraryExt, diag::FileResult, foundations::{Bytes, Datetime, Duration}, syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot}, text::{Font, FontBook}, utils::LazyHash};
use typst_kit::{files::{FileLoader, FileStore, FsRoot, SystemFiles}, fonts::FontStore, packages::SystemPackages};
use typst_layout::PagedDocument;
use typst_svg::SvgOptions;

fn main() {

    let world = World::new();

    let result =  typst::compile(&world);
    let wanings  = result.warnings;

    println!("{:#?}", wanings);
    let document: PagedDocument = result.output.unwrap();

    println!("dpc{:#?}", document.pages()[0]);

    let out = typst_svg::svg(&document.pages()[0], &SvgOptions { render_bleed: false, pretty: true });
    println!("{}", out)
}

pub struct World {
    lib: LazyHash<Library>,
    fonts: FontStore,
    main: FileId,
    files: FileStore<SystemFiles>
}

impl World {
    pub fn new() -> World {
        let files = FileStore::new(SystemFiles::new(FsRoot::new(".".into()), SystemPackages::new(Downloader)));

        let vroot = VirtualRoot::Project;
        let vpath = VirtualPath::new("test.typ").unwrap();
        let root = RootedPath::new(vroot, vpath);
        let main = FileId::new(root);

        println!("{:#?}", main);

        let mut fonts = FontStore::new();
        fonts.extend(typst_kit::fonts::system());
        
        World { 
            lib: Library::builder().build().into(),
            fonts,
            main,
            files
        }
    }
}


impl typst::World for World {
    fn library(&self) ->  &LazyHash<Library>  {
        &self.lib
    }

    fn book(&self) ->  &LazyHash<FontBook>  {
        self.fonts.book()
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source>  {
        println!("id: {:#?}", id);
        let a  =self.files.source(id);
        println!("{:#?}", a.clone().unwrap().text());
        a
    }

    fn file(&self, id: FileId) -> FileResult<Bytes>  {
        todo!()
    }

    fn font(&self, index: usize) -> Option<Font>  {
        let a = self.fonts.font(index);
        println!("font: {:#?}", a);

        a
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime>  {
        Datetime::from_ymd(1970, 1, 1)
    }
}

struct Downloader;
impl typst_kit::downloader::Downloader for Downloader {
    fn stream(
        &self,
        key: &dyn std::any::Any,
        url: &str,
    ) -> std::io::Result<(Option<usize>, Box<dyn std::io::prelude::Read>)>
    {
        std::io::Result::Err(Error::from(ErrorKind::AddrInUse))
    }
}