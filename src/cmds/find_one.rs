use super::val_converter::{doc_to_value, value_to_doc};
use crate::MongoPlugin;
use mongodb::bson::Document;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{
    Category, Example, LabeledError, Record, Signature, Spanned, SyntaxShape, Type, Value,
};

pub struct FindOne;

impl SimplePluginCommand for FindOne {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc find-one"
    }

    fn description(&self) -> &str {
        "find mongodb documents"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc find-one")
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
                description: "find a student with age `19`, in a `students` collection",
                example: "mongoc find-one {age: 19} -d 0 -c students",
                result: None,
            },
            Example {
                description: "find a teacher with name `John`, in a `teachers` collection",
                example: "mongoc find-one {name: John} -d 0 -c teachers",
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
        let result = db
            .collection::<Document>(&coll)
            .find_one(value_to_doc(query)?)
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;

        match result {
            None => Ok(Value::nothing(call.head)),
            Some(d) => Ok(doc_to_value(d, call.head)),
        }
    }
}
