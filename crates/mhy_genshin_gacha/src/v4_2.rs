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

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"{
        "info": {
            "export_timestamp": 1781688677,
            "export_time": "2026-06-15 12:00:00",
            "export_app": "Starward",
            "export_app_version": "0.17.0",
            "version": "v4.2"
        },
        "hk4e": [
            {
                "uid": "100000000",
                "timezone": 8,
                "lang": "zh-cn",
                "list": [
                    {
                        "uigf_gacha_type": "301",
                        "gacha_type": "301",
                        "id": "1667354760051767691",
                        "name": "班尼特",
                        "item_type": "武器",
                        "rank_type": "4",
                        "time": "2022-11-02 10:49:39",
                        "item_id": "10000032",
                        "count": "1"
                    }
                ]
            }
        ]
    }"#;

    #[test]
    fn test_deserialize() {
        let root: Uigf = serde_json::from_str(DATA).unwrap();
        assert_eq!(root.info.version.as_ref(), "v4.2");
        assert!(root.info.export_time.is_some());

        let hk4e = root.hk4e.unwrap();
        assert_eq!(hk4e.len(), 1);
        assert_eq!(hk4e[0].list[0].name.as_deref(), Some("班尼特"));
    }

    #[test]
    fn test_serialize_roundtrip() {
        let root: Uigf = serde_json::from_str(DATA).unwrap();
        let serialized = serde_json::to_string(&root).unwrap();
        let roundtrip: Uigf = serde_json::from_str(&serialized).unwrap();

        assert_eq!(roundtrip.info.version.as_ref(), "v4.2");
        assert!(roundtrip.info.export_time.is_some());
    }

    #[test]
    fn test_deserialize_without_export_time() {
        let json = r#"{
            "info": {
                "export_timestamp": 1781688677,
                "export_app": "Starward",
                "export_app_version": "0.17.0",
                "version": "v4.2"
            }
        }"#;

        let root: Uigf = serde_json::from_str(json).unwrap();
        assert!(root.info.export_time.is_none());
        assert!(root.hk4e.is_none());
    }
}
