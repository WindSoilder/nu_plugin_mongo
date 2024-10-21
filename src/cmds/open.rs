use crate::MongoPlugin;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Type, Value};

pub struct Open;

impl SimplePluginCommand for Open {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc open"
    }

    fn description(&self) -> &str {
        "open mongodb connection"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc open")
            .required("mongo-url", SyntaxShape::String, "mongodb url to connect")
            .input_output_type(Type::Nothing, Type::Int)
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Open a connection to mongodb",
            example: "mongoc open \"mongodb://localhost/db\"",
            result: None,
        }]
    }

    fn run(
        &self,
        plugin: &MongoPlugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let _ = engine.set_gc_disabled(true);
        let conn_str: String = call.req(0)?;
        let handler_id = plugin.connect(&conn_str)?;
        Ok(Value::int(handler_id.into(), call.head))
    }
}
