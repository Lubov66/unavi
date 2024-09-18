#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, ValueEnum};
use surrealdb::Surreal;
use tracing::Level;
use unavi_app::StartOptions;

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let search = location.search().unwrap();
    let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();

    let mut args = Args {
        debug_physics: false,
        log_level: LogLevel::default(),
        storage: Storage::Memory,
        xr: false,
    };

    if let Some(value) = params.get("debug-physics") {
        if let Ok(value) = value.parse() {
            args.debug_physics = value;
        }
    }

    if let Some(value) = params.get("log-level") {
        match value.as_str() {
            "info" => args.log_level = LogLevel::Info,
            "debug" => args.log_level = LogLevel::Debug,
            "trace" => args.log_level = LogLevel::Trace,
            _ => tracing::warn!("Unknown log-level: {}", value),
        }
    }

    let db = Surreal::new::<surrealdb::engine::local::IndxDb>("unavi")
        .await
        .expect("Failed to create SurrealDB.");

    let opts = args_to_options(args);

    unavi_app::start(db, opts).await;
}

#[cfg(target_family = "wasm")]
fn main() {}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Enables debug physics visuals.
    #[arg(long)]
    debug_physics: bool,

    /// Minimum log level.
    #[arg(long, default_value_t, value_enum)]
    log_level: LogLevel,

    #[arg(long, default_value = "filesystem")]
    storage: Storage,

    /// Enables XR mode.
    #[arg(long)]
    xr: bool,
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Storage {
    Filesystem,
    Memory,
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    use surrealdb::engine::local::{Mem, SurrealKV};
    use unavi_app::{dirs::get_project_dirs, update::check_for_updates};

    if let Err(e) = check_for_updates() {
        panic!("Failed to update: {}", e);
    };

    let args = Args::parse();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime");

    rt.block_on(async {
        let db = match args.storage {
            Storage::Filesystem => {
                let dirs = get_project_dirs();
                let db_path = dirs.data_dir();

                std::fs::create_dir_all(db_path).expect("Failed to create database dir.");

                Surreal::new::<SurrealKV>(db_path)
                    .await
                    .expect("Failed to create SurrealDB.")
            }
            Storage::Memory => Surreal::new::<Mem>(())
                .await
                .expect("Failed to create SurrealDB."),
        };

        let opts = args_to_options(args);
        unavi_app::start(db, opts).await;
    });
}

fn args_to_options(args: Args) -> StartOptions {
    let log_level = match args.log_level {
        LogLevel::Info => Level::INFO,
        LogLevel::Debug => Level::DEBUG,
        LogLevel::Trace => Level::TRACE,
    };

    StartOptions {
        debug_physics: args.debug_physics,
        log_level,
        xr: args.xr,
    }
}
