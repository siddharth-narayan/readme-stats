use typst::{
    Library, LibraryExt, diag::FileResult, foundations::{Bytes, Datetime, Dict, Duration}, syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot}, text::{Font, FontBook}, utils::LazyHash,
};
use typst_kit::{
    datetime::Time, downloader::SystemDownloader, files::{FileStore, FsRoot, SystemFiles}, fonts::{self, FontStore}, packages::SystemPackages,
};

pub struct World {
    lib: LazyHash<Library>,
    fonts: FontStore,
    main: FileId,
    file_store: FileStore<SystemFiles>,
}

impl World {
    pub fn new<T: AsRef<str>>(file_name: T, inputs: Dict) -> World {
        let file_store = FileStore::new(SystemFiles::new(
            FsRoot::new(".".into()),
            SystemPackages::new(SystemDownloader::new("")),
        ));

        let main_path = VirtualPath::new(file_name).unwrap();
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