use std::sync::Arc;

use anyhow::{bail, Result};
use capnp::capability::Promise;
use capnp_rpc::pry;
use dwn::{
    actor::{Actor, MessageBuilder},
    message::descriptor::Descriptor,
    store::{DataStore, MessageStore},
};
use tracing::{debug, error};
use wired_social::protocols::world_host::world_host_protocol_url;
use wired_world::world_server_capnp::world_server::{
    JoinParams, JoinResults, LeaveParams, LeaveResults, PlayerParams, PlayerResults, PlayersParams,
    PlayersResults, Server, TickrateParams, TickrateResults,
};

use crate::{
    commands::{SessionCommand, SessionMessage},
    global_context::GlobalContext,
};

pub struct WorldServer<D: DataStore, M: MessageStore> {
    pub actor: Arc<Actor<D, M>>,
    pub player_id: usize,
    pub context: Arc<GlobalContext>,
}

impl<D: DataStore + 'static, M: MessageStore + 'static> Server for WorldServer<D, M> {
    fn join(&mut self, params: JoinParams, mut results: JoinResults) -> Promise<(), capnp::Error> {
        let params = pry!(params.get());
        let record_id = pry!(pry!(params.get_record_id()).to_string());

        debug!("Request to join instance: {}", record_id);

        let actor = self.actor.clone();
        let context = self.context.clone();
        let player_id = self.player_id;
        let world_host_did = self.context.world_host_did.clone();

        Promise::from_future(async move {
            let mut success = results.get().init_success();

            match verify_instance(actor, world_host_did, record_id.clone()).await {
                Ok(_) => {
                    debug!("Instance {} verified.", record_id);

                    context
                        .sender
                        .send(SessionMessage {
                            command: SessionCommand::JoinInstance { id: record_id },
                            player_id,
                        })
                        .map_err(|e| {
                            error!("Send failed: {}", e);
                            capnp::Error::from_kind(capnp::ErrorKind::Failed)
                        })?;

                    success.set_success(());
                }
                Err(e) => {
                    let e = e.to_string();
                    debug!("Instance error {}", e);
                    success.init_error(e.len() as u32).push_str(&e);
                }
            };

            Ok(())
        })
    }

    fn leave(&mut self, params: LeaveParams, _: LeaveResults) -> Promise<(), capnp::Error> {
        let params = pry!(params.get());
        let record_id = pry!(pry!(params.get_record_id()).to_string());

        let context = self.context.clone();
        let player_id = self.player_id;

        Promise::from_future(async move {
            context
                .sender
                .send(SessionMessage {
                    command: SessionCommand::LeaveInstance { id: record_id },
                    player_id,
                })
                .map_err(|e| {
                    error!("Send failed: {}", e);
                    capnp::Error::from_kind(capnp::ErrorKind::Failed)
                })?;

            Ok(())
        })
    }

    fn players(&mut self, _: PlayersParams, _: PlayersResults) -> Promise<(), capnp::Error> {
        todo!();
    }

    fn player(&mut self, _: PlayerParams, _: PlayerResults) -> Promise<(), capnp::Error> {
        todo!();
    }

    fn tickrate(&mut self, _: TickrateParams, _: TickrateResults) -> Promise<(), capnp::Error> {
        todo!();
    }
}

/// Verifies the provided `record_id` is a valid instance.
async fn verify_instance(
    actor: Arc<Actor<impl DataStore, impl MessageStore>>,
    world_host_did: String,
    record_id: String,
) -> Result<()> {
    let read = actor
        .read_record(record_id)
        .target(world_host_did)
        .process()
        .await?;
    debug!("Found record {}", read.record.record_id);

    let descriptor = match &read.record.descriptor {
        Descriptor::RecordsWrite(d) => d,
        _ => bail!("Invalid descriptor type"),
    };

    if descriptor.protocol != Some(world_host_protocol_url()) {
        debug!("Invalid instance protocol: {:?}", descriptor.protocol);
        bail!("Invalid descriptor")
    }

    if descriptor.protocol_path != Some("instance".to_string()) {
        debug!(
            "Invalid instance protocol path: {:?}",
            descriptor.protocol_path
        );
        bail!("Invalid descriptor")
    }

    Ok(())
}
