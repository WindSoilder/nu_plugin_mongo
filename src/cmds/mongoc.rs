use crate::MongoPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, Type, Value};

#[derive(Clone)]
pub struct MongoCmd;

impl PluginCommand for MongoCmd {
    type Plugin = MongoPlugin;

    fn name(&self) -> &str {
        "mongoc"
    }

    fn description(&self) -> &str {
        "Operate with mongodb"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("mongoc")
            .category(Category::Database)
            .input_output_types(vec![(Type::Nothing, Type::String)])
    }

    fn extra_description(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce this help message."
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        Ok(PipelineData::Value(
            Value::string(engine.get_help()?, call.head),
            None,
        ))
    }
}
