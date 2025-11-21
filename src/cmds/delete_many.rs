use super::val_converter::value_to_doc;
use crate::MongoPlugin;
use mongodb::bson::Document;
use nu_plugin::{DynamicCompletionCall, EngineInterface, SimplePluginCommand};
use nu_protocol::{
    Category, DynamicSuggestion, Example, LabeledError, Record, Signature, Spanned, SyntaxShape,
    Type, Value, engine::ArgType,
};

pub struct DeleteMany;

impl SimplePluginCommand for DeleteMany {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc delete-many"
    }

    fn description(&self) -> &str {
        "delete many mongodb documents"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc delete-many")
            .optional("query object", SyntaxShape::Record(vec![]), "query object")
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
                description: "delete students with age `19`, in a `students` collection",
                example: "mongoc delete-many {age: 19} -d 0 -c students",
                result: None,
            },
            Example {
                description: "delete teachers with name `John`, in a `teachers` collection",
                example: "mongoc delete-many {name: John} -d 0 -c teachers",
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
        let query: Record = call.opt(0)?.unwrap_or_default();
        let coll = db.collection::<Document>(&coll);
        let delete_cmd = coll.delete_many(value_to_doc(query)?);
        delete_cmd
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
