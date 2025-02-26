use enum_as_inner::EnumAsInner;
use ffi_rpc::{
    self, abi_stable, async_trait, bincode,
    ffi_rpc_macro::{self, plugin_api},
};
use semver::Version;
use serde_repr::{Deserialize_repr, Serialize_repr};
use skynet_api::{HyUuid, uuid};
use std::{
    fmt::{self, Display},
    path::PathBuf,
};

pub use semver;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ID: HyUuid = HyUuid(uuid!("ce96ae04-6801-4ca4-b09d-a087e05f3783"));

#[derive(EnumAsInner, Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum System {
    Windows,
    Linux,
    OSX,
}

impl System {
    #[must_use]
    pub fn parse(str: &str) -> Option<Self> {
        let str = str.to_lowercase();
        if str.contains("windows") {
            Some(Self::Windows)
        } else if str.contains("linux") {
            Some(Self::Linux)
        } else if str.contains("macos") {
            Some(Self::OSX)
        } else {
            None
        }
    }
}

impl Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Windows => write!(f, "windows"),
            Self::Linux => write!(f, "linux"),
            Self::OSX => write!(f, "osx"),
        }
    }
}

#[derive(EnumAsInner, Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Arch {
    X86,
    X64,
    ARM,
    ARM64,
}

impl Arch {
    #[must_use]
    pub fn parse(str: &str) -> Option<Self> {
        let str = str.to_lowercase();
        if str.contains("x86_64") {
            Some(Self::X64)
        } else if str.contains("x86") {
            Some(Self::X86)
        } else if str.contains("aarch64") {
            Some(Self::ARM64)
        } else if str.contains("arm") {
            Some(Self::ARM)
        } else {
            None
        }
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X86 => write!(f, "x86"),
            Self::X64 => write!(f, "x64"),
            Self::ARM => write!(f, "arm"),
            Self::ARM64 => write!(f, "arm64"),
        }
    }
}

#[plugin_api(AgentService)]
pub trait Service: Send + Sync {
    async fn api_version() -> Version;
    async fn check_version(v: String) -> bool;
    async fn get_binary_name(sys: System, arch: Arch) -> PathBuf;
    async fn get_binary(sys: System, arch: Arch) -> Option<Vec<u8>>;
}
