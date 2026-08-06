use super::val_converter::value_to_doc;
use crate::MongoPlugin;
use mongodb::{IndexModel, bson::Document, options::IndexOptions};
use nu_plugin::{DynamicCompletionCall, EngineInterface, SimplePluginCommand};
use nu_protocol::{
    Category, DynamicSuggestion, Example, LabeledError, Record, Signature, Spanned, SyntaxShape,
    Type, Value, engine::ArgType,
};

pub struct CreateIndex;

impl SimplePluginCommand for CreateIndex {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc create-index"
    }

    fn description(&self) -> &str {
        "create a mongodb index"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc create-index")
            .required(
                "index object",
                SyntaxShape::Record(vec![].into()),
                "index key object",
            )
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
            .named("name", SyntaxShape::String, "index name", Some('n'))
            .input_output_type(Type::Nothing, Type::String)
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "create an index for `age` in a `students` collection",
                example: "mongoc create-index {age: 1} -c students",
                result: None,
            },
            Example {
                description: "create a named index for `age` in a `students` collection",
                example: "mongoc create-index {age: 1} -c students -n student_age_idx",
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
        let keys: Spanned<Record> = call.req(0)?;
        if keys.item.is_empty() {
            return Err(LabeledError::new("invalid index object")
                .with_label("index key object can't be empty", keys.span));
        }

        let name: Option<Spanned<String>> = call.get_flag("name")?;
        let options = match name {
            None => None,
            Some(name) => {
                if name.item.is_empty() {
                    return Err(LabeledError::new("invalid index name")
                        .with_label("index name can't be empty", name.span));
                }
                Some(IndexOptions::builder().name(name.item).build())
            }
        };
        let index = IndexModel::builder()
            .keys(value_to_doc(keys.item)?)
            .options(options)
            .build();
        let result = db
            .collection::<Document>(&coll)
            .create_index(index)
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;

        Ok(Value::string(result.index_name, call.head))
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
