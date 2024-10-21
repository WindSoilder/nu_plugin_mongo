use nu_plugin::{serve_plugin, MsgPackSerializer};

use nu_plugin_mongo::MongoPlugin;

fn main() {
    serve_plugin(&MongoPlugin::new(), MsgPackSerializer {})
}
