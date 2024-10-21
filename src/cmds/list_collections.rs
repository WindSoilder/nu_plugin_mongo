use crate::MongoPlugin;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Example, LabeledError, Signature, Spanned, SyntaxShape, Type, Value};

pub struct ListCollectionNames;

impl SimplePluginCommand for ListCollectionNames {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc list-colls"
    }

    fn description(&self) -> &str {
        "list all available collection names"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc list-colls")
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
        vec![
            Example {
                description: "list collection names under current database handle",
                example: "mongoc list-colls",
                result: None,
            },
            Example {
                description: "list collection names with given database handle",
                example: "mongoc list-colls -d 1",
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
        let result = db.list_collection_names().run().unwrap();
        let mut rows = vec![];
        for name in result {
            rows.push(Value::string(name, call.head))
        }
        Ok(Value::list(rows, call.head))
    }
}
