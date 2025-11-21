use crate::MongoPlugin;
use nu_plugin::{DynamicCompletionCall, EngineInterface, SimplePluginCommand};
use nu_protocol::{
    Category, DynamicSuggestion, Example, LabeledError, Signature, Spanned, SyntaxShape, Type,
    Value, engine::ArgType,
};

pub struct Remove;

impl SimplePluginCommand for Remove {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc remove"
    }

    fn description(&self) -> &str {
        "remove mongodb handles"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc remove")
            .required(
                "db-handle",
                SyntaxShape::Int,
                "database handle to remove, can get from `mongoc list`",
            )
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Remove handle 0",
            example: "mongoc remove 0",
            result: None,
        }]
    }

    fn run(
        &self,
        plugin: &MongoPlugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let handle_id: Spanned<i64> = call.req(0)?;
        plugin.remove_handle(handle_id.item as u8, handle_id.span)?;
        Ok(Value::nothing(call.head))
    }
    fn get_dynamic_completion(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        _call: DynamicCompletionCall,
        arg_type: ArgType,
        _experimental: nu_protocol::engine::ExperimentalMarker,
    ) -> Option<Vec<DynamicSuggestion>> {
        match arg_type {
            ArgType::Flag(name) if name == "collection" => {
                super::get_collection_names_at_current_handle(plugin)
            }
            _ => None,
        }
    }
}
