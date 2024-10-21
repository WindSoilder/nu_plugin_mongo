use crate::MongoPlugin;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{record, Category, LabeledError, Signature, Type, Value};

pub struct List;
impl SimplePluginCommand for List {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc list"
    }

    fn description(&self) -> &str {
        "list mongodb connections"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc list")
            .category(Category::Database)
            .input_output_type(
                Type::Nothing,
                Type::List(Box::new(Type::Record(Box::new([
                    ("id".to_string(), Type::Int),
                    ("addr".to_string(), Type::String),
                ])))),
            )
    }

    fn run(
        &self,
        plugin: &MongoPlugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let mut handles = plugin.list_handles();
        handles.sort_by_key(|k| k.0);
        let mut result = vec![];
        for (id, conn_str) in handles {
            result.push(Value::record(
                record! {
                "id" => Value::int(id.into(), call.head),
                "addr" => Value::string(conn_str, call.head)
                },
                call.head,
            ))
        }
        Ok(Value::list(result, call.head))
    }
}
