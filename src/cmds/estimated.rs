use crate::MongoPlugin;
use mongodb::bson::Document;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Example, LabeledError, Signature, Spanned, SyntaxShape, Type, Value};

pub struct Estimated;

impl SimplePluginCommand for Estimated {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc estimated"
    }

    fn description(&self) -> &str {
        "estimated mongodb documents count"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc estimated")
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
            .input_output_type(Type::Nothing, Type::record())
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "estimated students count with age `19`, in a `students` collection",
                example: "mongoc estimated {age: 19} -d 0 -c students",
                result: None,
            },
            Example {
                description:
                    "estimated teachers count with name `John`, in a `teachers` collection",
                example: "mongoc estimated {name: John} -d 0 -c teachers",
                result: None,
            },
            Example {
                description: "estimated teachers count with name `John`",
                example: "mongoc estimated {name: John} -d 0 -c teachers -s {\"age\": -1}",
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
        let estimated = coll.estimated_document_count();
        let result = estimated
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;
        Ok(Value::int(result as i64, call.head))
    }
}
