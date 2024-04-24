use anyhow::{bail, Result};
use wasm_component_layer::{
    AsContextMut, EnumType, Func, FuncType, Linker, Store, Value, ValueType,
};

use crate::scripting::{load::EngineBackend, StoreData};

const DEBUG: &str = "debug";
const INFO: &str = "info";
const WARN: &str = "warn";
const ERROR: &str = "error";

const LEVELS: [&str; 4] = [DEBUG, INFO, WARN, ERROR];

pub fn add_to_host(store: &mut Store<StoreData, EngineBackend>, linker: &mut Linker) -> Result<()> {
    let log_level = EnumType::new(None, LEVELS)?;

    let log = Func::new(
        store.as_context_mut(),
        FuncType::new([ValueType::Enum(log_level), ValueType::String], []),
        move |_ctx, args, _results| {
            let level = match &args[0] {
                Value::Enum(level) => LEVELS[level.discriminant()],
                _ => bail!("invalid arg type"),
            };

            let message = match &args[1] {
                Value::String(message) => message,
                _ => bail!("invalid arg type"),
            };

            let message = format!("Script: {}", message);

            match level {
                DEBUG => tracing::debug!("{}", message),
                INFO => tracing::info!("{}", message),
                WARN => tracing::warn!("{}", message),
                ERROR => tracing::error!("{}", message),
                _ => unreachable!(),
            };

            Ok(())
        },
    );

    let interface = linker.define_instance("wired:log/api".try_into()?)?;

    interface.define_func("log", log)?;

    Ok(())
}
