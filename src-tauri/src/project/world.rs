use crate::engine::TypstEngine;
use chrono::Datelike;
use comemo::Prehashed;
use std::cell::{OnceCell, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use typst::diag::{FileError, FileResult, PackageError, PackageResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::{Library, World};

pub struct ProjectWorld {
    root: PathBuf,
    engine: Arc<TypstEngine>,

    /// Map of slots, identified by [FileId]
    slots: RefCell<HashMap<FileId, PathSlot>>,

    /// This should be set upon project initialization. If the
    /// main source is set to [Option::None], then the compilation
    /// should not occur. Otherwise, the code will panic.
    main: Option<FileId>,
}

impl ProjectWorld {
    pub fn slot_update<P: AsRef<Path>>(
        &mut self,
        path: P,
        content: Option<String>,
    ) -> FileResult<FileId> {
        let vpath = VirtualPath::new(path);
        let id = FileId::new(None, vpath.clone());
        let mut slot = self.slot(id)?;

        if let Some(res) = slot.buffer.get_mut() {
            // TODO: Avoid cloning?
            let bytes = self.take_or_read_bytes(&vpath, content.clone())?;
            match res {
                Ok(b) => {
                    *b = bytes;
                }
                Err(_) => {
                    *res = Ok(bytes);
                }
            }
        };
        if let Some(res) = slot.source.get_mut() {
            let content = self.take_or_read(&vpath, content)?;
            match res {
                Ok(src) => {
                    // TODO: incremental edits
                    src.replace(&content);
                }
                Err(_) => {
                    *res = Ok(Source::new(id, content));
                }
            }
        };
        Ok(id)
    }

    pub fn set_main(&mut self, id: Option<FileId>) {
        self.main = id
    }

    pub fn set_main_path(&mut self, main: VirtualPath) {
        self.set_main(Some(FileId::new(None, main)))
    }

    pub fn is_main_set(&self) -> bool {
        self.main.is_some()
    }

    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            engine: Arc::new(TypstEngine::new()),
            slots: RefCell::default(),
            main: None,
        }
    }

    fn slot(&self, id: FileId) -> FileResult<RefMut<PathSlot>> {
        let mut path = PathBuf::new();
        let mut slots = self.slots.borrow_mut();
        if let Entry::Vacant(_) = &slots.entry(id) {
            let buf;
            let mut root = &self.root;
            if let Some(spec) = id.package() {
                buf = Self::prepare_package(spec)?;
                root = &buf;
            }

            // This will disallow paths outside of the root directory. Note that this will
            // still allow symlinks.
            path = id.vpath().resolve(root).ok_or(FileError::AccessDenied)?;
        }

        Ok(RefMut::map(slots, |slots| {
            slots.entry(id).or_insert_with(|| PathSlot {
                id,
                path,
                source: OnceCell::new(),
                buffer: OnceCell::new(),
            })
        }))
    }

    fn take_or_read(&self, vpath: &VirtualPath, content: Option<String>) -> FileResult<String> {
        if let Some(content) = content {
            return Ok(content);
        }

        let path = vpath.resolve(&self.root).ok_or(FileError::AccessDenied)?;
        fs::read_to_string(&path).map_err(|e| FileError::from_io(e, &path))
    }

    fn take_or_read_bytes(
        &self,
        vpath: &VirtualPath,
        content: Option<String>,
    ) -> FileResult<Bytes> {
        if let Some(content) = content {
            return Ok(Bytes::from(content.into_bytes()));
        }

        let path = vpath.resolve(&self.root).ok_or(FileError::AccessDenied)?;
        fs::read(&path)
            .map_err(|e| FileError::from_io(e, &path))
            .map(Bytes::from)
    }

    fn prepare_package(spec: &PackageSpec) -> PackageResult<PathBuf> {
        let subdir = format!(
            "typst/packages/{}/{}/{}",
            spec.namespace, spec.name, spec.version
        );

        if let Some(data_dir) = dirs::data_dir() {
            let dir = data_dir.join(&subdir);
            if dir.exists() {
                return Ok(dir);
            }
        }

        if let Some(cache_dir) = dirs::cache_dir() {
            let dir = cache_dir.join(&subdir);
            if dir.exists() {
                return Ok(dir);
            }
        }

        Err(PackageError::NotFound(spec.clone()))
    }
}

impl World for ProjectWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.engine.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.engine.fontbook
    }

    fn main(&self) -> Source {
        self.source(self.main.expect("the main file must be set"))
            .expect("unable to load the main file") // TODO: Handle this better
            .clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.slot(id)?.source()
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.slot(id)?.file()
    }

    fn font(&self, id: usize) -> Option<Font> {
        let slot = &self.engine.fonts[id];
        slot.font
            .get_or_init(|| {
                let data = fs::read(&slot.path).map(Bytes::from).ok()?;
                Font::new(data, slot.index)
            })
            .clone()
    }

    // TODO: Should probably cache this per compilation, to ensure consistent datetime throughout the document
    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let dt = match offset {
            None => chrono::Local::now().naive_local(),
            Some(o) => (chrono::Utc::now() + chrono::Duration::try_hours(o)?).naive_utc(),
        };
        Datetime::from_ymd(
            dt.year(),
            dt.month().try_into().ok()?,
            dt.day().try_into().ok()?,
        )
    }
}

struct PathSlot {
    id: FileId,
    path: PathBuf,
    source: OnceCell<FileResult<Source>>,
    buffer: OnceCell<FileResult<Bytes>>,
}

impl PathSlot {
    fn source(&self) -> FileResult<Source> {
        self.source
            .get_or_init(|| {
                let text = fs::read_to_string(&self.path)
                    .map_err(|e| FileError::from_io(e, &self.path))?;
                Ok(Source::new(self.id, text))
            })
            .clone()
    }

    fn file(&self) -> FileResult<Bytes> {
        // TODO: Unsure whether buffer should be implemented this way. This may cause a lot of memory usage on projects with a lot of large files.
        self.buffer
            .get_or_init(|| {
                fs::read(&self.path)
                    .map(Bytes::from)
                    .map_err(|e| FileError::from_io(e, &self.path))
            })
            .clone()
    }
}
