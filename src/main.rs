use std::path::PathBuf;

use typst::{Library, LibraryExt, World, syntax::{FileId, RootedPath, VirtualPath}, text::FontBook, utils::LazyHash};
use typst_kit::{files::{FileStore, FsRoot, SystemFiles}, packages::{FsPackages, SystemPackages}};
use typst_svg::SvgOptions;

fn main() {
    let fonts = typst_kit::fonts::FontStore::default().book();
    FileStore::new()
    typst::compile(world)
    typst_svg::svg(page, )
    println!("Hello, world!");

    let mut root = PathBuf::new();
    FileStore::new(SystemFiles::new(FsRoot::new(PathBuf::new()), FsPackages::new(".")));
    FileId::new(RootedPath::new(typst::syntax::VirtualRoot::Project, VirtualPath::new("/").unwrap()));
}

pub struct World {
    lib: LazyHash<Library>,
    fonts: LazyHash<FontBook>,
    main: FileId,
}

impl World {
    pub fn new() -> World {

    }
}
impl typst::World for World {
    fn library(&self) ->  &LazyHash<Library>  {
        let library = Library::builder().build();
    }

    #[doc = " Metadata about all known fonts."]
    fn book(&self) ->  &LazyHash<FontBook>  {
        todo!()
    }

    #[doc = " Get the file id of the main source file."]
    fn main(&self) -> FileId {
        todo!()
    }

    #[doc = " Try to access the specified file location as a source file."]
    fn source(&self,id: FileId) -> FileResult<Source>  {
        todo!()
    }

    #[doc = " Try to access the specified file."]
    #[doc = ""]
    #[doc = " For file locations for which [`source`](Self::source) succeeds, this"]
    #[doc = " should also succeed. The [`Bytes`] can be cheaply created as a view into"]
    #[doc = " an existing [`Source`] through [`Bytes::from_string`]."]
    fn file(&self,id: FileId) -> FileResult<Bytes>  {
        todo!()
    }

    #[doc = " Try to access the font with the given index in the font book."]
    #[doc = ""]
    #[doc = " Note that the index is not guaranteed to be in bounds of the font book"]
    #[doc = " returned by this world\'s `book()` function. This is the case because"]
    #[doc = " this function may be invoked with indices from an outdated or different"]
    #[doc = " font book during incremental compilation validation."]
    fn font(&self,index: usize) -> Option<Font>  {
        todo!()
    }

    #[doc = " Get the current date."]
    #[doc = ""]
    #[doc = " If no offset is specified, the local date should be chosen. Otherwise,"]
    #[doc = " the UTC date should be chosen with the corresponding offset."]
    #[doc = ""]
    #[doc = " If this function returns `None`, Typst\'s `datetime` function will"]
    #[doc = " return an error."]
    fn today(&self,offset: Option<Duration>) -> Option<Datetime>  {
        todo!()
    }
}