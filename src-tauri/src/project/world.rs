use crate::engine::TypstEngine;
use comemo::Prehashed;
use elsa::FrozenVec;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use typst::diag::{FileError, FileResult};
use typst::eval::Library;
use typst::font::{Font, FontBook};
use typst::syntax::{Source, SourceId};
use typst::util::Buffer;
use typst::World;

pub struct ProjectWorld {
    root: PathBuf,
    engine: Arc<TypstEngine>,

    /// Map of source ids, identified by the canonical path.
    paths: RefCell<HashMap<PathBuf, FileResult<SourceId>>>,

    /// A list of sources, identified by its id
    sources: FrozenVec<Box<Source>>,

    /// This should be set upon project initialization. If the
    /// main source is set to [Option::None], then the compilation
    /// should not occur. Otherwise, the code will panic.
    main: Option<SourceId>,
}

impl ProjectWorld {
    pub fn slot_update(&mut self, path: &Path, content: Option<String>) -> FileResult<SourceId> {
        let mut paths = self.paths.borrow_mut();
        match paths.entry(path.to_path_buf()) {
            Entry::Occupied(mut o) => match o.get() {
                Ok(id) => {
                    let sources = self.sources.as_mut();
                    let src = &mut sources[id.clone().into_u16() as usize];
                    if let Ok(content) = content.ok_or_else(|| fs::read_to_string(&path).ok()) {
                        src.replace(content)
                    }
                    Ok(id.clone())
                }
                Err(_) => o.insert(self.insert(path, content)).clone(),
            },
            Entry::Vacant(v) => v.insert(self.insert(path, content)).clone(),
        }
    }

    pub fn set_main(&mut self, source: Option<SourceId>) {
        self.main = source
    }

    pub fn try_set_main<P: AsRef<Path>>(&mut self, main: P) -> FileResult<()> {
        self.slot_update(main.as_ref(), None)
            .map(|source| self.set_main(Some(source)))
    }

    pub fn is_main_set(&self) -> bool {
        self.main.is_some()
    }

    /// Retrieves an existing path slot or inserts a new one.
    /// Inserting a new one will assign a source id and will
    /// load the file's content from the file system.
    fn slot(&self, path: &Path) -> FileResult<SourceId> {
        let path = self
            .root
            .join(path)
            .canonicalize()
            .map_err(|_| FileError::NotFound(path.into()))?;
        if !path.starts_with(&self.root) {
            return Err(FileError::AccessDenied);
        }
        let mut paths = self.paths.borrow_mut();
        paths
            .entry(path.clone())
            .or_insert_with(|| self.insert(&path, None))
            .clone()
    }

    fn insert<P: AsRef<Path>>(&self, path: P, content: Option<String>) -> FileResult<SourceId> {
        let content = match content {
            Some(content) => content,
            None => fs::read_to_string(&path).map_err(|e| FileError::from_io(e, path.as_ref()))?,
        };

        let sources = &self.sources;
        let id = SourceId::from_u16(sources.len() as u16);
        let source = Source::new(id, path.as_ref(), content);
        sources.push(Box::new(source));

        Ok(id)
    }

    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            engine: Arc::new(TypstEngine::new()),
            paths: RefCell::default(),
            sources: Default::default(),
            main: None,
        }
    }
}

impl World for ProjectWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.engine.library
    }

    fn main(&self) -> &Source {
        self.source(self.main.expect("Main file must be set"))
    }

    fn resolve(&self, path: &Path) -> FileResult<SourceId> {
        self.slot(path)
    }

    fn source(&self, id: SourceId) -> &Source {
        &self.sources[id.into_u16() as usize]
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.engine.fontbook
    }

    fn font(&self, id: usize) -> Option<Font> {
        let slot = &self.engine.fonts[id];
        slot.font
            .get_or_init(|| {
                let data = fs::read(&slot.path).map(Buffer::from).ok()?;
                Font::new(data, slot.index)
            })
            .clone()
    }

    fn file(&self, path: &Path) -> FileResult<Buffer> {
        let path = self
            .root
            .join(path)
            .canonicalize()
            .map_err(|_| FileError::NotFound(path.into()))?;

        if !path.starts_with(&self.root) {
            return Err(FileError::AccessDenied);
        }

        fs::read(&path)
            .map(Buffer::from)
            .map_err(|e| FileError::from_io(e, path.as_ref()))
    }
}
