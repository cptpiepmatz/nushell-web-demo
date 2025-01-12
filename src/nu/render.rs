use crate::util::{Theme, Themed};

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use nu_protocol::{
    ast::{self, Argument, Expression},
    engine::{Call, Command, EngineState, Stack},
    Config, FromValue, PipelineData, Span, SpanId, Spanned, Type, Value,
};

pub struct RenderedData {
    pub table: Themed<String>,
    pub json: String,
    pub html: Themed<String>,
}

static THEMES: OnceLock<Themed<HashMap<String, Value>>> = OnceLock::new();

impl RenderedData {
    pub fn render(pipeline_data: PipelineData, engine_state: &EngineState) -> RenderedData {
        let themes = Self::initialize_themes();
        let mut stack = Self::initialize_stack(&themes);
        let value = pipeline_data.into_value(Span::unknown()).unwrap();

        let table_call = Call::new(Span::unknown());
        let table = Themed {
            light: Self::render_via_command(
                nu_command::Table,
                engine_state,
                &mut stack.light,
                &table_call,
                value.clone(),
            ),
            dark: Self::render_via_command(
                nu_command::Table,
                engine_state,
                &mut stack.dark,
                &table_call,
                value.clone(),
            ),
        };

        let json_call = Call::new(Span::unknown());
        let json = Self::render_via_command(
            nu_command::ToJson,
            engine_state,
            &mut stack.light,
            &json_call,
            value.clone(),
        );

        let html_call = Self::html_call();
        let html = Themed {
            light: Self::render_via_command(
                nu_cmd_extra::ToHtml,
                engine_state,
                &mut stack.light,
                &html_call.light,
                value.clone(),
            ),
            dark: Self::render_via_command(
                nu_cmd_extra::ToHtml,
                engine_state,
                &mut stack.dark,
                &html_call.dark,
                value,
            ),
        };

        RenderedData { table, json, html }
    }

    fn initialize_themes() -> &'static Themed<HashMap<String, Value>> {
        THEMES.get_or_init(|| {
            let themes = nuon::from_nuon(include_str!("../../themes.nuon"), None).unwrap();
            let mut themes = themes.into_record().unwrap();
            Themed {
                light: HashMap::from_value(themes.remove("light_theme").unwrap()).unwrap(),
                dark: HashMap::from_value(themes.remove("dark_theme").unwrap()).unwrap(),
            }
        })
    }

    fn initialize_stack(themes: &Themed<HashMap<String, Value>>) -> Themed<Stack> {
        let mut stack = Themed::pair(Stack::new());
        stack.update(|stack, theme| {
            stack.config = Some(Arc::new(Config {
                use_ansi_coloring: true.into(),
                color_config: match theme {
                    Theme::Light => themes.light.clone(),
                    Theme::Dark => themes.dark.clone(),
                },
                ..Default::default()
            }))
        });
        stack
    }

    fn render_via_command(
        command: impl Command,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        value: Value,
    ) -> String {
        let result =
            command.run(engine_state, stack, call, PipelineData::Value(value, None)).unwrap();
        result.into_value(Span::unknown()).unwrap().coerce_string().unwrap()
    }

    fn html_call() -> Themed<Call<'static>> {
        Themed::pair(ast::Call::new(Span::unknown())).map(|mut call, theme| {
            let partial = Argument::Named((
                Spanned { item: "partial".to_string(), span: Span::unknown() },
                None,
                None,
            ));

            match theme {
                Theme::Light => call.arguments = vec![partial],
                Theme::Dark => {
                    let dark = Argument::Named((
                        Spanned { item: "dark".to_string(), span: Span::unknown() },
                        None,
                        None,
                    ));
                    call.arguments = vec![partial, dark];
                }
            }

            Call {
                head: call.head,
                decl_id: call.decl_id,
                inner: nu_protocol::engine::CallImpl::AstBox(Box::new(call)),
            }
        })
    }

    pub fn empty() -> RenderedData {
        RenderedData {
            table: Themed::pair(String::new()),
            json: String::new(),
            html: Themed::pair(String::new()),
        }
    }
}
