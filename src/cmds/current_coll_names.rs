use crate::MongoPlugin;
use nu_protocol::Span;
pub fn get_collection_names_at_current_handle(plugin: &MongoPlugin) -> Option<Vec<String>> {
    let current_handle = plugin.get_current();
    if let Ok(current_handle) = current_handle {
        get_collection_names(current_handle, plugin)
    } else {
        None
    }
}

fn get_collection_names(handle: u8, plugin: &MongoPlugin) -> Option<Vec<String>> {
    plugin
        .get_handle(handle, Span::unknown())
        .map(|db| db.list_collection_names().run().ok())
        .unwrap_or(None)
}
