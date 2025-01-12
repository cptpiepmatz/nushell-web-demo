use std::{collections::HashMap, sync::{Arc, OnceLock}};

use nu_protocol::{
    ast::{self, Argument}, engine::{Call, Command, EngineState, Stack}, Config, FromValue, PipelineData, Span, Spanned, Value
};

pub struct RenderedData {
    pub table: Themed<String>,
    pub json: String,
    pub html: String,
}

pub struct Themed<T> {
    pub light: T,
    pub dark: T,
}

pub enum Theme {
    Light,
    Dark,
}

impl<T> Themed<T> {
    fn pair(initial: T) -> Self
    where
        T: Clone,
    {
        Self { light: initial.clone(), dark: initial }
    }

    fn update(&mut self, f: impl Fn(&mut T, Theme)) {
        f(&mut self.light, Theme::Light);
        f(&mut self.dark, Theme::Dark);
    }
}

static THEMES: OnceLock<Themed<HashMap<String, Value>>> = OnceLock::new();

impl RenderedData {
    pub fn render(pipeline_data: PipelineData, engine_state: &EngineState) -> RenderedData {
        let themes = THEMES.get_or_init(|| {
            let themes = nuon::from_nuon(include_str!("../../themes.nuon"), None).unwrap();
            let mut themes = themes.into_record().unwrap();
            Themed {
                light: HashMap::from_value(themes.remove("light_theme").unwrap()).unwrap(),
                dark: HashMap::from_value(themes.remove("dark_theme").unwrap()).unwrap(),
            }
        });

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

        let call = Call::new(Span::unknown());
        let value = pipeline_data.into_value(Span::unknown()).unwrap();

        let table = nu_command::Table;
        let table_light = table
            .run(&engine_state, &mut stack.light, &call, PipelineData::Value(value.clone(), None))
            .unwrap();
        let table_light = table_light.into_value(Span::unknown()).unwrap();
        let table_light = table_light.coerce_string().unwrap();
        let table_dark = table
        .run(&engine_state, &mut stack.dark, &call, PipelineData::Value(value.clone(), None))
        .unwrap();
    let table_dark = table_dark.into_value(Span::unknown()).unwrap();
    let table_dark = table_dark.coerce_string().unwrap();

        let json = nu_command::ToJson;
        let json = json
            .run(&engine_state, &mut stack.light, &call, PipelineData::Value(value.clone(), None))
            .unwrap();
        let json = json.into_value(Span::unknown()).unwrap();
        let json = json.into_string().unwrap();

        let mut call = ast::Call::new(Span::unknown());
        let argument = Argument::Named((
            Spanned { item: "partial".to_string(), span: Span::unknown() },
            None,
            None,
        ));
        call.arguments = vec![argument];
        let call = Call::from(&call);
        let html = nu_cmd_extra::ToHtml;
        let html = html
            .run(&engine_state, &mut stack.light, &call, PipelineData::Value(value, None))
            .unwrap();
        let html = html.into_value(Span::unknown()).unwrap();
        let html = html.into_string().unwrap();

        RenderedData { table: Themed {
            light: table_light,
            dark: table_dark,
        }, json, html }
    }

    pub fn empty() -> RenderedData {
        RenderedData {
            table: Themed::pair(String::new()),
            json: String::new(),
            html: String::new(),
        }
    }
}
