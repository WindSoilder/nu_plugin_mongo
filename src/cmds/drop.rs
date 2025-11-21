use crate::MongoPlugin;
use mongodb::bson::Document;
use nu_plugin::{DynamicCompletionCall, EngineInterface, SimplePluginCommand};
use nu_protocol::{
    Category, DynamicSuggestion, Example, LabeledError, Signature, Spanned, SyntaxShape, Type,
    Value, engine::ArgType,
};

pub struct Drop;

impl SimplePluginCommand for Drop {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc drop"
    }

    fn description(&self) -> &str {
        "drop a mongodb collection"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc drop")
            .required_named(
                "collection",
                SyntaxShape::String,
                "collection name",
                Some('c'),
            )
            .named(
                "db-handle",
                SyntaxShape::Int,
                "database handle, can get from `mongoc list`",
                Some('d'),
            )
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "drop `students` collection",
                example: "mongoc drop -c students",
                result: None,
            },
            Example {
                description: "drop `teachers` collection",
                example: "mongoc drop -c teachers",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        plugin: &MongoPlugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let db_handle: Option<Spanned<i64>> = call.get_flag("db-handle")?;
        let db = match db_handle {
            None => plugin.get_handle(plugin.get_current()?, call.head)?,
            Some(db_handle) => plugin.get_handle(db_handle.item as u8, db_handle.span)?,
        };
        let coll: String = call
            .get_flag("collection")?
            .expect("already check existed.");
        let coll = db.collection::<Document>(&coll);
        let drop_cmd = coll.drop();
        drop_cmd
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;

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
