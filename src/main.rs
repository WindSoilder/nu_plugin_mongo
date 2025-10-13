use nu_plugin::{MsgPackSerializer, serve_plugin};

use nu_plugin_mongo::MongoPlugin;

fn main() {
    serve_plugin(&MongoPlugin::new(), MsgPackSerializer {})
}
