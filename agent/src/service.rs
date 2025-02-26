use std::{fs, path::PathBuf};

use skynet_api::ffi_rpc::{
    self, async_trait, bincode, ffi_rpc_macro::plugin_impl_trait, registry::Registry,
};
use skynet_api_agent::{Arch, System, semver::Version};

use crate::{PLUGIN_INSTANCE, Plugin};

#[plugin_impl_trait]
impl skynet_api_agent::Service for Plugin {
    async fn api_version(&self, _: &Registry) -> Version {
        Version::parse(skynet_api_agent::VERSION).unwrap()
    }

    async fn check_version(&self, _: &Registry, v: String) -> bool {
        let v = if let Ok(v) = Version::parse(&v) {
            v
        } else {
            return false;
        };

        Version::parse(env!("CARGO_PKG_VERSION")).unwrap() > v
    }

    async fn get_binary_name(&self, _: &Registry, sys: System, arch: Arch) -> PathBuf {
        let suffix = if sys.is_windows() { ".exe" } else { "" };
        PLUGIN_INSTANCE
            .path
            .get()
            .unwrap()
            .join("bin")
            .join(format!("agent_{sys}_{arch}{suffix}"))
    }

    async fn get_binary(&self, _: &Registry, sys: System, arch: Arch) -> Option<Vec<u8>> {
        let suffix = if sys.is_windows() { ".exe" } else { "" };
        fs::read(
            PLUGIN_INSTANCE
                .path
                .get()
                .unwrap()
                .join("bin")
                .join(format!("agent_{sys}_{arch}{suffix}")),
        )
        .ok()
    }
}
