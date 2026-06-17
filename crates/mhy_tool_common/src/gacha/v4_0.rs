use std::borrow::Cow;

use super::ExportTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportInfo<'a> {
    #[serde(borrow)]
    pub export_timestamp: ExportTimestamp<'a>,
    #[serde(borrow)]
    pub export_app: Cow<'a, str>,
    #[serde(borrow)]
    pub export_app_version: Cow<'a, str>,
    #[serde(borrow)]
    pub version: Cow<'a, str>,
}

impl<'a> From<super::v4_2::ExportInfo<'a>> for ExportInfo<'a> {
    fn from(value: super::v4_2::ExportInfo<'a>) -> Self {
        ExportInfo {
            export_timestamp: value.export_timestamp,
            export_app: value.export_app,
            export_app_version: value.export_app_version,
            version: value.version,
        }
    }
}
