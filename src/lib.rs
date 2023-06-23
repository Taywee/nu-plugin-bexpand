use crate::Example;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape, Type, Value};

impl Plugin for Example {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("str bexpand")
            .input_output_types(vec![(Value::String, Value::List(Box::new(Value::String)))])
            .usage("Bash-style brace expansion")
            .plugin_examples(vec![PluginExample {
                example: "'~/config/nushell/{env,config,plugin}.nu | str bexpand".into(),
                description: "Get a list of standard nushell config items".into(),
                result: None,
            }])
            .category(Category::Strings)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // You can use the name to identify what plugin signature was called
        match name {
            "str bexpand" => self.test1(call, input),
        }
    }
}
