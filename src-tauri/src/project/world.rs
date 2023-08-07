use crate::engine::TypstEngine;
use chrono::Datelike;
use comemo::Prehashed;
use std::cell::{OnceCell, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use typst::diag::{FileError, FileResult};
use typst::eval::{Bytes, Datetime, Library};
use typst::font::{Font, FontBook};
use typst::syntax::{FileId, Source};
use typst::util::PathExt;
use typst::World;

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
        let id = FileId::new(None, path.as_ref());
        let mut slot = self.slot(id)?;

        match slot.buffer.get_mut() {
            // Only update existing buffers. There is no need to insert new buffers
            Some(res) => {
                // TODO: Avoid cloning?
                let bytes = self.take_or_read_bytes(&path, content.clone())?;
                match res {
                    Ok(b) => {
                        *b = bytes;
                    }
                    Err(_) => {
                        *res = Ok(bytes);
                    }
                }
            }
            None => {}
        };
        match slot.source.get_mut() {
            // Only update existing sources. There is no need to insert new sources
            Some(res) => {
                let content = self.take_or_read(&path, content)?;
                match res {
                    Ok(src) => {
                        // TODO: incremental edits
                        src.replace(content);
                    }
                    Err(_) => {
                        *res = Ok(Source::new(id, content));
                    }
                }
            }
            None => {}
        };
        Ok(id)
    }

    pub fn set_main(&mut self, id: Option<FileId>) {
        self.main = id
    }

    pub fn set_main_path<P: AsRef<Path>>(&mut self, main: P) {
        self.set_main(Some(FileId::new(None, main.as_ref())))
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
            // This will disallow paths outside of the root directory. Note that this will
            // still allow symlinks.
            path = self
                .root
                .join_rooted(id.path())
                .ok_or(FileError::AccessDenied)?;
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

    fn take_or_read<P: AsRef<Path>>(&self, path: P, content: Option<String>) -> FileResult<String> {
        if let Some(content) = content {
            return Ok(content);
        }

        let path = self
            .root
            .join_rooted(path.as_ref())
            .ok_or(FileError::AccessDenied)?;
        fs::read_to_string(&path).map_err(|e| FileError::from_io(e, &path))
    }

    fn take_or_read_bytes<P: AsRef<Path>>(
        &self,
        path: P,
        content: Option<String>,
    ) -> FileResult<Bytes> {
        if let Some(content) = content {
            return Ok(Bytes::from(content.into_bytes()));
        }

        let path = self
            .root
            .join_rooted(path.as_ref())
            .ok_or(FileError::AccessDenied)?;
        fs::read(&path)
            .map_err(|e| FileError::from_io(e, &path))
            .map(Bytes::from)
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
            Some(o) => (chrono::Utc::now() + chrono::Duration::hours(o)).naive_utc(),
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
