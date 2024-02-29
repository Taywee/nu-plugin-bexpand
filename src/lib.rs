use std::borrow::Cow;

use bexpand::Expression;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Span, Type, Value};

pub struct Bexpand;

fn bexpand(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = input.span();
    let output: Result<Vec<Cow<str>>, _> = match input {
        Value::String { val, .. } => {
            let val = val.as_str();
            let expression: Expression = match val.try_into() {
                Ok(e) => e,
                Err(s) => {
                    return Err(LabeledError {
                        label: "Brace expression failed to parse".into(),
                        msg: s,
                        span: Some(span),
                    })
                }
            };

            expression.into_iter().collect()
        }
        Value::List { vals, .. } => {
            let exprs: Result<Vec<_>, _> = vals
                .into_iter()
                .map(|val| match val {
                    Value::String { val, .. } => {
                        let val = val.as_str();
                        let expression: Expression = match val.try_into() {
                            Ok(e) => e,
                            Err(s) => {
                                return Err(LabeledError {
                                    label: "Brace expression failed to parse".into(),
                                    msg: s,
                                    span: Some(span),
                                })
                            }
                        };

                        Ok(expression.into_iter())
                    }
                    v => {
                        return Err(LabeledError {
                            label: "Input must be string".into(),
                            msg: format!("Input type was {}", input.get_type()),
                            span: Some(v.span()),
                        })
                    }
                })
                .collect();
            exprs?.into_iter().flatten().collect()
        }

        v => {
            return Err(LabeledError {
                label: "Input must be string".into(),
                msg: format!("Input type was {}", input.get_type()),
                span: Some(v.span()),
            })
        }
    };

    let output = match output {
        Ok(o) => o
            .into_iter()
            .map(|s| Value::string(s.into_owned(), call.head))
            .collect(),
        Err(e) => {
            return Err(LabeledError {
                label: "Expression failed to generate".into(),
                msg: e.to_string(),
                span: Some(input.span()),
            })
        }
    };

    Ok(Value::list(output, call.head))
}

impl Plugin for Bexpand {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("str bexpand")
            .input_output_types(vec![
                (Type::String, Type::List(Box::new(Type::String))),
                (
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::String)),
                ),
            ])
            .usage("Bash-style brace expansion")
            .plugin_examples(vec![PluginExample {
                example: "'~/.config/nushell/{env,config,plugin}.nu' | str bexpand".into(),
                description: "Get a list of standard nushell config items".into(),
                result: Some(Value::List {
                    vals: vec![
                        Value::String {
                            val: "~/.config/nushell/env.nu".into(),
                            internal_span: Span::new(0, 0),
                        },
                        Value::String {
                            val: "~/.config/nushell/config.nu".into(),
                            internal_span: Span::new(0, 0),
                        },
                        Value::String {
                            val: "~/.config/nushell/plugin.nu".into(),
                            internal_span: Span::new(0, 0),
                        },
                    ],
                    internal_span: Span::new(0, 0),
                }),
            }])
            .category(Category::Strings)]
    }

    fn run(
        &mut self,
        name: &str,
        _: &Option<Value>,
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
