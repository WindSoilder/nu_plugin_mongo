use super::val_converter::value_to_doc;
use crate::MongoPlugin;
use mongodb::bson::Document;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{
    Category, Example, LabeledError, Record, Signature, Spanned, SyntaxShape, Type, Value,
};

pub struct DeleteOne;

impl SimplePluginCommand for DeleteOne {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc delete-one"
    }

    fn description(&self) -> &str {
        "delete one mongodb document"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc delete-one")
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
                description: "delete a student with age `19`, in a `students` collection",
                example: "mongoc delete-one {age: 19} -d 0 -c students",
                result: None,
            },
            Example {
                description: "delete a teacher with name `John`, in a `teachers` collection",
                example: "mongoc delete-one {name: John} -d 0 -c teachers",
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
        let delete_cmd = coll.delete_one(value_to_doc(query)?);
        delete_cmd
            .run()
            .map_err(|e| LabeledError::new(format!("{e}")))?;

        Ok(Value::nothing(call.head))
    }

    fn get_completion(
        &self,
        plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        flag_name: &str,
    ) -> Option<Vec<String>> {
        match flag_name {
            "collection" => super::get_collection_names_at_current_handle(plugin),
            _ => None,
        }
    }
}
