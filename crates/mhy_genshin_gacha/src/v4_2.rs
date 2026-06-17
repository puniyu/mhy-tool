pub use super::v4::{Hk4eGachaType, Item, Player};

use mhy_tool_common::gacha::v4_2::ExportInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uigf<'a> {
    #[serde(borrow)]
    pub info: ExportInfo<'a>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hk4e: Option<Vec<Player<'a>>>,
}

impl<'a> From<super::v4_0::Uigf<'a>> for Uigf<'a> {
    fn from(value: super::v4_0::Uigf<'a>) -> Self {
        Uigf {
            info: value.info.into(),
            hk4e: value.hk4e,
        }
    }
}

impl<'a> From<super::v4_1::Uigf<'a>> for Uigf<'a> {
    fn from(value: super::v4_1::Uigf<'a>) -> Self {
        Uigf {
            info: value.info.into(),
            hk4e: value.hk4e,
        }
    }
}

impl<'a> From<super::Uigf<'a>> for Uigf<'a> {
    fn from(value: super::Uigf<'a>) -> Self {
        Uigf {
            info: value.info,
            hk4e: value.hk4e,
        }
    }
}
