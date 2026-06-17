pub mod v4;
pub mod v4_0;
pub mod v4_1;
pub mod v4_2;

use mhy_tool_common::gacha::v4_2::ExportInfo;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uigf<'a> {
    #[serde(borrow)]
    pub info: ExportInfo<'a>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hk4e: Option<Vec<v4::Player<'a>>>,
}

impl<'a> From<v4_0::Uigf<'a>> for Uigf<'a> {
    fn from(value: v4_0::Uigf<'a>) -> Self {
        Uigf {
            info: value.info.into(),
            hk4e: value.hk4e,
        }
    }
}

impl<'a> From<v4_1::Uigf<'a>> for Uigf<'a> {
    fn from(value: v4_1::Uigf<'a>) -> Self {
        Uigf {
            info: value.info.into(),
            hk4e: value.hk4e,
        }
    }
}

impl<'a> From<v4_2::Uigf<'a>> for Uigf<'a> {
    fn from(value: v4_2::Uigf<'a>) -> Self {
        Uigf {
            info: value.info,
            hk4e: value.hk4e,
        }
    }
}

impl<'a> Uigf<'a> {
    pub fn to_v4_0(self) -> v4_0::Uigf<'a> {
        v4_0::Uigf::from(self)
    }

    pub fn to_v4_1(self) -> v4_1::Uigf<'a> {
        v4_1::Uigf::from(self)
    }

    pub fn to_v4_2(self) -> v4_2::Uigf<'a> {
        v4_2::Uigf::from(self)
    }
}

impl Uigf<'_> {
    pub fn load_from_str(s: &str) -> Result<Uigf<'_>, serde_json::Error> {
        serde_json::from_str(s)
    }
}
