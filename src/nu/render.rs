use nu_protocol::{
    ast::{self, Argument},
    engine::{Call, Command, EngineState, Stack},
    PipelineData, Span, Spanned,
};

pub struct RenderedData {
    pub table: String,
    pub json: String,
    pub html: String,
}

impl RenderedData {
    pub fn render(pipeline_data: PipelineData, engine_state: &EngineState) -> RenderedData {
        let mut stack = Stack::new();
        let call = Call::new(Span::unknown());

        let value = pipeline_data.into_value(Span::unknown()).unwrap();

        let table = nu_command::Table;
        let table = table
            .run(&engine_state, &mut stack, &call, PipelineData::Value(value.clone(), None))
            .unwrap();
        let table = table.into_value(Span::unknown()).unwrap();
        let table = table.into_string().unwrap();

        let json = nu_command::ToJson;
        let json = json
            .run(&engine_state, &mut stack, &call, PipelineData::Value(value.clone(), None))
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
        let html =
            html.run(&engine_state, &mut stack, &call, PipelineData::Value(value, None)).unwrap();
        let html = html.into_value(Span::unknown()).unwrap();
        let html = html.into_string().unwrap();

        RenderedData { table, json, html }
    }

    pub fn empty() -> RenderedData {
        RenderedData { table: String::new(), json: String::new(), html: String::new() }
    }
}
