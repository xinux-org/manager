use crate::value::XmgrValue;

#[derive(Debug)]
pub struct XmgrPackage {
    pub name: String,
    pub value: XmgrPackageValue,
}

#[derive(Debug)]
pub enum XmgrPackageValue {
    FromPkgs,
    FromFlake { src: String, package: String },
    Skip,
}

impl From<XmgrValue> for XmgrPackageValue {
    fn from(value: XmgrValue) -> Self {
        match value {
            XmgrValue::PackageFromPkgs => XmgrPackageValue::FromPkgs,
            XmgrValue::PackageFromFlake { src, package } => {
                XmgrPackageValue::FromFlake { src, package }
            }
            _ => XmgrPackageValue::Skip,
        }
    }
}
