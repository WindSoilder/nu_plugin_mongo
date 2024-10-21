use crate::MongoPlugin;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, LabeledError, Signature, Spanned, SyntaxShape, Type, Value};

pub struct Select;

impl SimplePluginCommand for Select {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc select"
    }

    fn description(&self) -> &str {
        "select current mongodb handle"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc select")
            .required(
                "db-handle",
                SyntaxShape::Int,
                "database handle to seelct, can get from `mongoc list`",
            )
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Database)
    }

    fn run(
        &self,
        plugin: &MongoPlugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let handle_id: Spanned<i64> = call.req(0)?;
        plugin.select_handle(handle_id.item as u8, handle_id.span)?;
        Ok(Value::nothing(call.head))
    }
}
