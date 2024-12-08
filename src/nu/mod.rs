use nu_protocol::{
    debugger::WithoutDebug,
    engine::{EngineState, Stack, StateDelta, StateWorkingSet},
    CompileError, ParseError, PipelineData, ShellError, Span, Value,
};
use std::fmt::Debug;
use thiserror::Error;

pub mod render;

#[allow(clippy::let_and_return)] // i like it here
pub fn initial_engine_state() -> EngineState {
    let engine_state = nu_cmd_lang::create_default_context();

    let engine_state = nu_command::add_shell_command_context(engine_state);
    let engine_state = nu_cmd_extra::add_extra_command_context(engine_state);

    let mut engine_state = engine_state;
    engine_state.add_env_var("PWD".to_string(), Value::string("/", Span::unknown()));

    engine_state
}

pub fn execute(
    code: &str,
    engine_state: &mut EngineState,
    stack: &mut Stack,
    name: &str,
) -> Result<PipelineData, ExecuteError> {
    let code = code.as_bytes();
    let mut working_set = StateWorkingSet::new(engine_state);
    let block = nu_parser::parse(&mut working_set, Some(name), code, false);

    // TODO: report parse warnings

    if let Some(error) = working_set.parse_errors.into_iter().next() {
        return Err(ExecuteError::Parse {
            error,
            delta: working_set.delta,
        });
    }

    if let Some(error) = working_set.compile_errors.into_iter().next() {
        return Err(ExecuteError::Compile {
            error,
            delta: working_set.delta,
        });
    }

    engine_state.merge_delta(working_set.delta)?;
    let res =
        nu_engine::eval_block::<WithoutDebug>(engine_state, stack, &block, PipelineData::Empty)?;
    Ok(res)
}

#[derive(Error)]
pub enum ExecuteError {
    #[error("{error}")]
    Parse {
        #[source]
        error: ParseError,
        /// Delta of the working set.
        ///
        /// By keeping this delta around we later can update another working
        /// set with and with that correctly source the parsing issues.
        delta: StateDelta,
    },

    #[error("{error}")]
    Compile {
        #[source]
        error: CompileError,
        /// Delta of the working set.
        ///
        /// By keeping this delta around we later can update another working
        /// set with and with that correctly source the parsing issues.
        delta: StateDelta,
    },

    #[error(transparent)]
    Shell(#[from] ShellError),
}

impl Debug for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse { error, delta } => f
                .debug_struct("Parse")
                .field("error", error)
                .field("delta", &format_args!("[StateDelta]"))
                .finish(),
            Self::Compile { error, delta } => f
                .debug_struct("Compile")
                .field("error", error)
                .field("delta", &format_args!("[StateDelta]"))
                .finish(),
            Self::Shell(arg0) => f.debug_tuple("Shell").field(arg0).finish(),
        }
    }
}
