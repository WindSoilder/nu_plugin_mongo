use super::val_converter::value_to_doc;
use crate::MongoPlugin;
use mongodb::bson::Document;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{
    Category, Example, LabeledError, Record, Signature, Spanned, SyntaxShape, Type, Value,
};

pub struct Count;

impl SimplePluginCommand for Count {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc count"
    }

    fn description(&self) -> &str {
        "count mongodb documents"
    }

    fn extra_description(&self) -> &str {
        "count documents might be slow, if you want to estimate document count, you can run `mongoc estimated`"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc count")
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
            .input_output_type(Type::Nothing, Type::record())
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "count students with age `19`, in a `students` collection",
                example: "mongoc count {age: 19} -d 0 -c students",
                result: None,
            },
            Example {
                description: "count teachers with name `John`, in a `teachers` collection",
                example: "mongoc count {name: John} -d 0 -c teachers",
                result: None,
            },
            Example {
                description: "count teachers with name `John`",
                example: "mongoc count {name: John} -d 0 -c teachers -s {\"age\": -1}",
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
        let counts = coll.count_documents(value_to_doc(query)?);
        let result = counts
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;
        Ok(Value::int(result as i64, call.head))
    }
}
