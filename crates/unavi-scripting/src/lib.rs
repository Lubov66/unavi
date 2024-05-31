use std::sync::{Arc, RwLock};

use bevy::prelude::*;

use self::{asset::Wasm, load::WasmStores, resource_table::ResourceTable};

mod asset;
mod execution;
mod host;
mod load;
mod resource_table;
mod script;
mod unavi_system;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(asset::WasmLoader)
            .init_asset::<Wasm>()
            .init_non_send_resource::<WasmStores>()
            .add_systems(Startup, unavi_system::spawn_unavi_system)
            .add_systems(
                FixedUpdate,
                (
                    (
                        host::wired_gltf::query::query_node_data,
                        execution::init_scripts,
                        execution::update_scripts,
                    )
                        .chain(),
                    host::wired_gltf::handler::handle_wired_gltf_actions,
                    load::load_scripts,
                ),
            );
    }
}

#[derive(Bundle)]
struct ScriptBundle {
    name: Name,
    wasm: Handle<Wasm>,
}

#[derive(Default)]
pub struct StoreData {
    pub resource_table: Arc<RwLock<ResourceTable>>,
}

/// Marks an entity as being owned by the provided entity.
///
/// For example, entities spawned in by a script are owned by that script.
/// The object the script belongs to owns the script.
/// The player that spawned in the object owns the object.
#[derive(Component, Deref)]
pub struct Ownership(pub Entity);
