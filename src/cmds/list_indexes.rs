use crate::MongoPlugin;
use bson::ser::to_document;
use mongodb::bson::Document;
use nu_plugin::{DynamicCompletionCall, EngineInterface, SimplePluginCommand};
use nu_protocol::{
    Category, DynamicSuggestion, Example, LabeledError, Signature, Spanned, SyntaxShape, Type,
    Value, engine::ArgType, record,
};

pub struct ListIndexes;

impl SimplePluginCommand for ListIndexes {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc list-indexes"
    }

    fn description(&self) -> &str {
        "find mongodb documents"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc list-indexes")
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
            .input_output_type(Type::Nothing, Type::table())
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "list indexes for collection `students`",
            example: "mongoc list-indexes students",
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
        let db_handle: Option<Spanned<i64>> = call.get_flag("db-handle")?;
        let db = match db_handle {
            None => plugin.get_handle(plugin.get_current()?, call.head)?,
            Some(db_handle) => plugin.get_handle(db_handle.item as u8, db_handle.span)?,
        };
        let coll: String = call
            .get_flag("collection")?
            .expect("already check existed.");
        let result = db
            .collection::<Document>(&coll)
            .list_indexes()
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;

        let mut rows = vec![];
        for doc in result {
            let doc = doc.map_err(|e| LabeledError::new(format!("{e}")))?;
            let rec = Value::record(
                record! {
                    "key" => Value::string(doc.keys.to_string(), call.head),
                    "options" => match doc.options {
                        None => Value::nothing(call.head),
                        Some(opt) => Value::string(to_document(&opt).unwrap().to_string(), call.head),
                    }
                },
                call.head,
            );
            rows.push(rec)
        }
        Ok(Value::list(rows, call.head))
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
