use std::borrow::Cow;

use bexpand::Expression;
use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, ErrorLabel, Example, LabeledError, Signature, Span, Type, Value};

fn bexpand(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = input.span();
    let output: Result<Vec<Cow<str>>, _> = match input {
        Value::String { val, .. } => {
            let val = val.as_str();
            let expression: Expression = match val.try_into() {
                Ok(e) => e,
                Err(s) => {
                    return Err(LabeledError {
                        labels: vec![ErrorLabel {
                            text: "Brace expression failed to parse".into(),
                            span: span,
                        }],
                        msg: s,
                        code: None,
                        url: None,
                        help: None,
                        inner: vec![],
                    });
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
                                    labels: vec![ErrorLabel {
                                        text: "Brace expression failed to parse".into(),
                                        span: span,
                                    }],
                                    msg: s,
                                    code: None,
                                    url: None,
                                    help: None,
                                    inner: vec![],
                                });
                            }
                        };

                        Ok(expression.into_iter())
                    }
                    v => {
                        return Err(LabeledError {
                            labels: vec![ErrorLabel {
                                text: "Input must be string".into(),
                                span: v.span(),
                            }],
                            msg: format!("Input type was {}", input.get_type()),
                            code: None,
                            url: None,
                            help: None,
                            inner: vec![],
                        });
                    }
                })
                .collect();
            exprs?.into_iter().flatten().collect()
        }

        v => {
            return Err(LabeledError {
                labels: vec![ErrorLabel {
                    text: "Input must be string".into(),
                    span: v.span(),
                }],
                msg: format!("Input type was {}", input.get_type()),
                code: None,
                url: None,
                help: None,
                inner: vec![],
            });
        }
    };

    let output = match output {
        Ok(o) => o
            .into_iter()
            .map(|s| Value::string(s.into_owned(), call.head))
            .collect(),
        Err(e) => {
            return Err(LabeledError {
                labels: vec![ErrorLabel {
                    text: "Expression failed to generate".into(),
                    span: input.span(),
                }],
                msg: e.to_string(),
                code: None,
                url: None,
                help: None,
                inner: vec![],
            });
        }
    };

    Ok(Value::list(output, call.head))
}

pub struct BexpandPlugin;

impl Plugin for BexpandPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Bexpand)]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}

pub struct Bexpand;

impl SimplePluginCommand for Bexpand {
    type Plugin = BexpandPlugin;

    fn signature(&self) -> Signature {
        Signature::new("str bexpand")
            .input_output_types(vec![
                (Type::String, Type::List(Box::new(Type::String))),
                (
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::String)),
                ),
            ])
            .description("Bash-style brace expansion")
            .category(Category::Strings)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // You can use the name to identify what plugin signature was called
        bexpand(call, input)
    }

    fn name(&self) -> &str {
        "str bexpand"
    }

    fn description(&self) -> &str {
        "Does bash-style brace expansion"
    }
    fn examples(&self) -> Vec<Example> {
        vec![Example {
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
        }]
    }
}
