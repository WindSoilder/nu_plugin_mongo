use super::val_converter::value_to_doc;
use crate::MongoPlugin;
use mongodb::bson::{Bson, Document};
use nu_plugin::{DynamicCompletionCall, EngineInterface, SimplePluginCommand};
use nu_protocol::{
    Category, DynamicSuggestion, Example, LabeledError, Record, Signature, Spanned, SyntaxShape,
    Type, Value, engine::ArgType,
};

pub struct DropIndex;

impl SimplePluginCommand for DropIndex {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc drop-index"
    }

    fn description(&self) -> &str {
        "drop a mongodb index"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc drop-index")
            .optional(
                "index object",
                SyntaxShape::Record(vec![].into()),
                "index key object; used to derive the index name when --name is omitted",
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
            .input_output_type(Type::Nothing, Type::Nothing)
            .category(Category::Database)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "drop an index by key object from a `students` collection",
                example: "mongoc drop-index {age: 1} -c students",
                result: None,
            },
            Example {
                description: "drop a named index from a `students` collection",
                example: "mongoc drop-index -c students -n student_age_idx",
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

        let name: Option<Spanned<String>> = call.get_flag("name")?;
        let keys: Option<Spanned<Record>> = call.opt(0)?;
        let index_name = match name {
            Some(name) => {
                if name.item.is_empty() {
                    return Err(LabeledError::new("invalid index name")
                        .with_label("index name can't be empty", name.span));
                }
                name.item
            }
            None => {
                let Some(keys) = keys else {
                    return Err(LabeledError::new("missing index name")
                        .with_label("provide --name or an index key object", call.head));
                };
                if keys.item.is_empty() {
                    return Err(LabeledError::new("invalid index object")
                        .with_label("index key object can't be empty", keys.span));
                }
                default_index_name(&value_to_doc(keys.item)?)
            }
        };

        db.collection::<Document>(&coll)
            .drop_index(index_name)
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

fn default_index_name(keys: &Document) -> String {
    keys.iter()
        .map(|(key, value)| format!("{key}_{}", index_key_value_name(value)))
        .collect::<Vec<_>>()
        .join("_")
}

fn index_key_value_name(value: &Bson) -> String {
    match value {
        Bson::String(value) => value.clone(),
        value => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::default_index_name;
    use mongodb::bson::doc;

    #[test]
    fn derives_default_index_name_from_keys() {
        let keys = doc! {
            "age": 1,
            "name": "text",
        };

        assert_eq!(default_index_name(&keys), "age_1_name_text");
    }
}
