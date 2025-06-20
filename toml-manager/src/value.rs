use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum XmgrValue {
    HostPlatform(String),
    PackageFromPkgs,
    PackageFromFlake { src: String, package: String },
}
