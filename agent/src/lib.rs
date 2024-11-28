use std::{path::PathBuf, sync::OnceLock};

use skynet_api::{
    ffi_rpc::{
        self,
        abi_stable::prefix_type::PrefixTypeTrait,
        async_ffi, async_trait, bincode,
        ffi_rpc_macro::{
            plugin_impl_call, plugin_impl_instance, plugin_impl_root, plugin_impl_trait,
        },
        registry::Registry,
    },
    plugin::{PluginError, PluginStatus, Request, Response},
    request::Router,
    service::SResult,
    Skynet,
};

mod service;

#[plugin_impl_instance(|| Plugin{
    path: Default::default(),
})]
#[plugin_impl_root]
#[plugin_impl_call(skynet_api::plugin::api::PluginApi, skynet_api_agent::Service)]
struct Plugin {
    path: OnceLock<PathBuf>,
}

#[plugin_impl_trait]
impl skynet_api::plugin::api::PluginApi for Plugin {
    async fn on_load(
        &self,
        _: &Registry,
        skynet: Skynet,
        runtime_path: PathBuf,
    ) -> SResult<Skynet> {
        let _ = self.path.set(runtime_path);
        Ok(skynet)
    }

    async fn on_register(&self, _: &Registry, _skynet: Skynet, r: Vec<Router>) -> Vec<Router> {
        r
    }

    async fn on_route(&self, _: &Registry, _name: String, _req: Request) -> SResult<Response> {
        Err(PluginError::NoSuchRoute.into())
    }

    async fn on_translate(&self, _: &Registry, str: String, _lang: String) -> String {
        str
    }

    async fn on_unload(&self, _: &Registry, _status: PluginStatus) {}
}
