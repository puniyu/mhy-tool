use std::borrow::Cow;

use super::{ExportTimestamp, option_datetime_serde};
use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportInfo<'a> {
    #[serde(borrow)]
    pub export_timestamp: ExportTimestamp<'a>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(with = "option_datetime_serde")]
    pub export_time: Option<DateTime>,
    #[serde(borrow)]
    pub export_app: Cow<'a, str>,
    #[serde(borrow)]
    pub export_app_version: Cow<'a, str>,
    #[serde(borrow)]
    pub version: Cow<'a, str>,
}

impl<'a> From<super::v4_0::ExportInfo<'a>> for ExportInfo<'a> {
    fn from(value: super::v4_0::ExportInfo<'a>) -> Self {
        ExportInfo {
            export_timestamp: value.export_timestamp,
            export_time: None,
            export_app: value.export_app,
            export_app_version: value.export_app_version,
            version: value.version,
        }
    }
}
