use typst::{Library, LibraryExt, diag::FileResult, foundations::{Bytes, Datetime, Duration}, syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot}, text::{Font, FontBook}, utils::LazyHash};
use typst_kit::{files::{FileStore, FsRoot, SystemFiles}, fonts::FontStore, packages};

fn main() {
}

pub struct World {
    lib: LazyHash<Library>,
    fonts: FontStore,
    main: FileId,
}

impl World {
    pub fn new() -> World {
        let vroot = VirtualRoot::Project;
        let vpath = VirtualPath::new(".").unwrap();
        let root = RootedPath::new(vroot, vpath);

        // let fonts = typst_assets::fonts().flat_map(|d| Font::iter(Bytes::new(d))).collect();
        // let font_book = FontBook::from_fonts(&fonts).into();
        let fonts = FontStore::default();

        World { 
            lib: Library::builder().build().into(),
            fonts,
            main: FileId::new(root)
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
        panic!("nah")
    }

    fn file(&self, id: FileId) -> FileResult<Bytes>  {
        todo!()
    }

    fn font(&self, index: usize) -> Option<Font>  {
        self.fonts.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime>  {
        Datetime::from_ymd(1970, 1, 1)
    }
}