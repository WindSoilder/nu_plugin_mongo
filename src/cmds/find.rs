use super::val_converter::{doc_to_value, value_to_doc};
use crate::MongoPlugin;
use mongodb::{bson::Document, options::FindOptions};
use nu_plugin::SimplePluginCommand;
use nu_protocol::{
    Category, Example, LabeledError, Record, Signature, Spanned, SyntaxShape, Type, Value,
};

pub struct Find;

impl SimplePluginCommand for Find {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc find"
    }

    fn description(&self) -> &str {
        "find mongodb documents"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc find")
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
            .named(
                "limit",
                SyntaxShape::Int,
                "limit rows to return, default is 10",
                Some('l'),
            )
            .named(
                "sort",
                SyntaxShape::Record(vec![]),
                "sort option",
                Some('s'),
            )
            .input_output_type(Type::Nothing, Type::table())
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "find documents in collection `students`",
                example: "mongoc find {} -d 0 -c students",
                result: None,
            },
            Example {
                description: "find `teachers` with name `John`, returns 300 rows at max",
                example: "mongoc find {name: John} -d 0 -c teachers -l 300",
                result: None,
            },
            Example {
                description: "find `teachers` with name `John`, sort by age ascending",
                example: "mongoc find {name: John} -d 0 -c teachers -s {\"age\": 1}",
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
        let limit: Spanned<i64> = call.get_flag("limit")?.unwrap_or(Spanned {
            item: 10,
            span: call.head,
        });
        if limit.item.is_negative() {
            return Err(
                LabeledError::new("get invalid number").with_label("can't be negative", limit.span)
            );
        }
        let limit = limit.item;
        let query: Record = call.opt(0)?.unwrap_or_default();
        let sort_options: Option<Record> = call.get_flag("sort")?;
        let coll = db.collection::<Document>(&coll);
        let mut find = coll.find(value_to_doc(query)?).limit(limit);
        if let Some(sort_opt) = sort_options {
            find = find.with_options(
                FindOptions::builder()
                    .sort(Some(value_to_doc(sort_opt)?))
                    .build(),
            )
        };
        let result = find.run().map_err(|e| LabeledError::new(format!("{e}")))?;

        let mut rows = vec![];
        for doc in result {
            let doc = doc.map_err(|e| LabeledError::new(format!("{e}")))?;
            rows.push(doc_to_value(doc, call.head))
        }
        Ok(Value::list(rows, call.head))
    }
}
