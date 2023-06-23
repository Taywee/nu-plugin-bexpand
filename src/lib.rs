use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Type, Value};

pub struct Bexpand;

fn bexpand(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let input = match input {
        Value::String { val, .. } => val.as_str(),
        _ => {
            return Err(LabeledError {
                label: "Input must be string".into(),
                msg: format!("Input type was {}", input.get_type()),
                span: dbg!(input.span().ok()),
            })
        }
    };

    todo!("parse and expand brace expression");

    Ok(Value::List {
        vals: vec![Value::String {
            val: input.into(),
            span: call.head,
        }],
        span: call.head,
    })
}

impl Plugin for Bexpand {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("str bexpand")
            .input_output_types(vec![(Type::String, Type::List(Box::new(Type::String)))])
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
            "str bexpand" => bexpand(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
