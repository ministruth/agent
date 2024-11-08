use actix_cloud::{async_trait, state::GlobalState};
use skynet_api::{create_plugin, plugin::Plugin, Result, Skynet};
use skynet_api_agent::{Service, ID, VERSION};
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

static SERVICE: OnceLock<Arc<Service>> = OnceLock::new();

#[derive(Debug, Default)]
struct Agent;

#[async_trait]
impl Plugin for Agent {
    fn on_load(
        &self,
        path: PathBuf,
        mut skynet: Box<Skynet>,
        state: Box<GlobalState>,
    ) -> (Box<Skynet>, Box<GlobalState>, Result<()>) {
        let _ = SERVICE.set(Arc::new(Service::new(
            path,
            env!("CARGO_PKG_VERSION").to_owned(),
        )));
        skynet
            .shared_api
            .set(&ID, VERSION, Box::new(SERVICE.get().unwrap().to_owned()));
        (skynet, state, Ok(()))
    }
}

create_plugin!(Agent, Agent::default);
