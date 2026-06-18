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

#[cfg(test)]
mod tests {
    use super::*;

    const V4_0_DATA: &str = r#"{
        "info": {
            "export_timestamp": 1781688677,
            "export_app": "Starward",
            "export_app_version": "0.17.0",
            "version": "v4.0"
        },
        "hk4e": [{
            "uid": "100000000",
            "timezone": 8,
            "list": [{
                "uigf_gacha_type": "301",
                "gacha_type": "301",
                "id": "1667354760051767691",
                "name": "班尼特",
                "item_type": "武器",
                "rank_type": "4",
                "time": "2022-11-02 10:49:39",
                "item_id": "10000032",
                "count": "1"
            }]
        }]
    }"#;

    const V4_2_DATA: &str = r#"{
        "info": {
            "export_timestamp": 1781688677,
            "export_time": "2026-06-15 12:00:00",
            "export_app": "Starward",
            "export_app_version": "0.17.0",
            "version": "v4.2"
        },
        "hk4e": [{
            "uid": "100000000",
            "timezone": 8,
            "list": [{
                "uigf_gacha_type": "301",
                "gacha_type": "301",
                "id": "1667354760051767691",
                "name": "班尼特",
                "item_type": "武器",
                "rank_type": "4",
                "time": "2022-11-02 10:49:39",
                "item_id": "10000032",
                "count": "1"
            }]
        }]
    }"#;

    #[test]
    fn test_load_from_str_v4_0() {
        let root = Uigf::load_from_str(V4_0_DATA).unwrap();
        assert_eq!(root.info.version.as_ref(), "v4.0");
        let hk4e = root.hk4e.unwrap();
        assert_eq!(hk4e[0].list[0].name.as_deref(), Some("班尼特"));
    }

    #[test]
    fn test_load_from_str_v4_2() {
        let root = Uigf::load_from_str(V4_2_DATA).unwrap();
        assert_eq!(root.info.version.as_ref(), "v4.2");
    }

    #[test]
    fn test_v4_0_to_latest() {
        let v4_0: v4_0::Uigf = serde_json::from_str(V4_0_DATA).unwrap();
        let latest = Uigf::from(v4_0);
        assert_eq!(latest.info.version.as_ref(), "v4.0");
        assert!(latest.hk4e.is_some());
    }

    #[test]
    fn test_v4_1_to_latest() {
        let v4_1: v4_1::Uigf = serde_json::from_str(V4_0_DATA).unwrap();
        let latest = Uigf::from(v4_1);
        assert_eq!(latest.info.version.as_ref(), "v4.0");
    }

    #[test]
    fn test_v4_2_to_latest() {
        let v4_2: v4_2::Uigf = serde_json::from_str(V4_2_DATA).unwrap();
        let latest = Uigf::from(v4_2);
        assert_eq!(latest.info.version.as_ref(), "v4.2");
        assert!(latest.hk4e.is_some());
    }

    #[test]
    fn test_latest_to_v4_0_roundtrip() {
        let root: Uigf = serde_json::from_str(V4_0_DATA).unwrap();
        let v4_0 = root.to_v4_0();
        assert_eq!(v4_0.info.version.as_ref(), "v4.0");
        assert_eq!(v4_0.hk4e.as_ref().unwrap()[0].list.len(), 1);
    }

    #[test]
    fn test_latest_to_v4_1_roundtrip() {
        let root: Uigf = serde_json::from_str(V4_0_DATA).unwrap();
        let v4_1 = root.to_v4_1();
        assert_eq!(v4_1.info.version.as_ref(), "v4.0");
    }

    #[test]
    fn test_latest_to_v4_2_roundtrip() {
        let root: Uigf = serde_json::from_str(V4_2_DATA).unwrap();
        let v4_2 = root.to_v4_2();
        assert_eq!(v4_2.info.version.as_ref(), "v4.2");
        assert!(v4_2.info.export_time.is_some());
    }

    #[test]
    fn test_cross_version_conversion() {
        let v4_0: v4_0::Uigf = serde_json::from_str(V4_0_DATA).unwrap();
        let v4_2 = v4_2::Uigf::from(v4_0);
        assert_eq!(v4_2.info.version.as_ref(), "v4.0");
        assert!(v4_2.info.export_time.is_none());
    }
}
