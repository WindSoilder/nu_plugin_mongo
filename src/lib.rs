mod cmds;
use cmds::*;
use mongodb::sync::Database;
use nu_plugin::{Plugin, PluginCommand};
use nu_protocol::{LabeledError, Span};
use std::collections::HashMap;
use std::sync::RwLock;

struct Handle {
    pub(crate) inner: HashMap<u8, (Database, String)>,
    pub(crate) current: u8,
}

impl Handle {
    fn new() -> Self {
        Self {
            inner: HashMap::default(),
            current: 0,
        }
    }
}
pub struct MongoPlugin {
    handlers: RwLock<Handle>,
}

impl Default for MongoPlugin {
    fn default() -> Self {
        Self {
            handlers: RwLock::new(Handle::new()),
        }
    }
}

impl MongoPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&self, conn_str: &str) -> Result<u8, LabeledError> {
        let conn = mongodb::sync::Client::with_uri_str(conn_str)
            .map_err(|err| LabeledError::new(format!("{err}")))
            .map(|c| c.default_database());
        match conn {
            Err(e) => Err(e),
            Ok(conn) => match conn {
                None => Err(LabeledError::new("No default database in connection url")),
                Some(db) => {
                    let mut write_guard = self.handlers.write().expect("write lock should success");
                    let id = write_guard.inner.len() as u8;
                    write_guard.inner.insert(id, (db, conn_str.to_string()));
                    write_guard.current = id;
                    Ok(id)
                }
            },
        }
    }

    pub fn list_handles(&self) -> Vec<(u8, String)> {
        let read_guard = self.handlers.read().expect("read lock should success");
        let mut result = vec![];
        for (id, (_, conn_str)) in read_guard.inner.iter() {
            result.push((*id, conn_str.to_string()));
        }
        result
    }

    pub fn get_handle(&self, id: u8, span: Span) -> Result<Database, LabeledError> {
        let read_guard = self.handlers.read().expect("read lock should success");
        let result = read_guard.inner.get(&id).ok_or_else(|| {
            LabeledError::new("database handle doesn't exist")
                .with_label("not existed database handle", span)
                .with_help("You can run `mongoc list` to list all available handles, or `mongoc open` to open a new handle")
        })?;
        Ok(result.0.clone())
    }

    pub fn remove_handle(&self, id: u8, span: Span) -> Result<(), LabeledError> {
        let mut write_guard = self.handlers.write().expect("write lock should success");
        write_guard.inner.remove(&id).ok_or_else(|| {
            LabeledError::new("database handle doesn't exist")
                .with_label("not existed database handle", span)
                .with_help("You can run `mongoc list` to list all available handles, or `mongoc open` to open a new handle")
        })?;
        // if remove current handle, reset the id.
        if write_guard.current == id {
            let max_id = write_guard.inner.keys().max().unwrap_or(&0);
            write_guard.current = *max_id;
        }
        Ok(())
    }

    pub fn select_handle(&self, id: u8, span: Span) -> Result<(), LabeledError> {
        let mut write_guard = self.handlers.write().expect("read lock should success");
        if !write_guard.inner.contains_key(&id) {
            let err = LabeledError::new("database handle doesn't exist")
                .with_label("not existed database handle", span)
                .with_help("You can run `mongoc list` to list all available handles, or `mongoc open` to open a new handle");
            return Err(err);
        }
        write_guard.current = id;
        Ok(())
    }

    pub fn get_current(&self) -> Result<u8, LabeledError> {
        let read_guard = self.handlers.read().expect("read lock should success");
        if read_guard.inner.is_empty() {
            return Err(LabeledError::new("no database handles available")
                .with_help("You can run `mongoc open` to open a new handle"));
        }
        Ok(read_guard.current)
    }
}

impl Plugin for MongoPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(Open),
            Box::new(List),
            Box::new(Find),
            Box::new(FindOne),
            Box::new(Drop),
            Box::new(DeleteOne),
            Box::new(DeleteMany),
            Box::new(Remove),
            Box::new(MongoCmd),
            Box::new(Select),
            Box::new(ListCollectionNames),
            Box::new(ListIndexes),
            Box::new(Count),
            Box::new(Estimated),
        ]
    }
}
