pub use super::v4::{Hk4eGachaType, Item, Player};

use mhy_tool_common::gacha::v4_1::ExportInfo;
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
            info: value.info,
            hk4e: value.hk4e,
        }
    }
}

impl<'a> From<super::v4_2::Uigf<'a>> for Uigf<'a> {
    fn from(value: super::v4_2::Uigf<'a>) -> Self {
        Uigf {
            info: value.info.into(),
            hk4e: value.hk4e,
        }
    }
}

impl<'a> From<super::Uigf<'a>> for Uigf<'a> {
    fn from(value: super::Uigf<'a>) -> Self {
        Uigf {
            info: value.info.into(),
            hk4e: value.hk4e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mhy_tool_common::gacha::UigfGachaType;

    const DATA: &str = r#"{
        "info": {
            "export_timestamp": 1781688677,
            "export_app": "Starward",
            "export_app_version": "0.17.0",
            "version": "v4.0"
        },
        "hk4e": [
            {
                "uid": "100000000",
                "timezone": 8,
                "lang": "zh-cn",
                "list": [
                    {
                        "uigf_gacha_type": "200",
                        "uid": "100000000",
                        "id": "1666980360000203591",
                        "gacha_type": "200",
                        "name": "弹弓",
                        "item_type": "角色",
                        "rank_type": "3",
                        "time": "2022-10-29 02:21:11",
                        "item_id": "15304",
                        "count": "1",
                        "lang": "zh-cn"
                    },
                    {
                        "uigf_gacha_type": "200",
                        "uid": "100000000",
                        "id": "1667271960000184191",
                        "gacha_type": "200",
                        "name": "神射手之誓",
                        "item_type": "角色",
                        "rank_type": "3",
                        "time": "2022-11-01 11:04:27",
                        "item_id": "15302",
                        "count": "1",
                        "lang": "zh-cn"
                    },
                    {
                        "uigf_gacha_type": "301",
                        "uid": "100000000",
                        "id": "1667354760051767391",
                        "gacha_type": "301",
                        "name": "弹弓",
                        "item_type": "角色",
                        "rank_type": "3",
                        "time": "2022-11-02 10:49:39",
                        "item_id": "15304",
                        "count": "1",
                        "lang": "zh-cn"
                    },
                    {
                        "uigf_gacha_type": "301",
                        "uid": "100000000",
                        "id": "1667354760051767691",
                        "gacha_type": "301",
                        "name": "班尼特",
                        "item_type": "武器",
                        "rank_type": "4",
                        "time": "2022-11-02 10:49:39",
                        "item_id": "10000032",
                        "count": "1",
                        "lang": "zh-cn"
                    }
                ]
            }
        ]
    }"#;

    #[test]
    fn test_deserialize_real_data() {
        let root: Uigf = serde_json::from_str(DATA).unwrap();

        assert_eq!(root.info.version.as_ref(), "v4.0");
        assert_eq!(root.info.export_app.as_ref(), "Starward");

        let hk4e = root.hk4e.unwrap();
        assert_eq!(hk4e.len(), 1);

        let player = &hk4e[0];
        assert_eq!(player.uid.value(), 100000000);
        assert_eq!(player.list.len(), 4);

        let first = &player.list[0];
        assert_eq!(first.gacha_type, Hk4eGachaType::A200);
        assert_eq!(first.uigf_gacha_type, UigfGachaType::A200);
        assert_eq!(first.name.as_deref(), Some("弹弓"));

        let bennett = &player.list[3];
        assert_eq!(bennett.name.as_deref(), Some("班尼特"));
        assert_eq!(bennett.rank_type.as_deref(), Some("4"));
    }

    #[test]
    fn test_serialize_roundtrip() {
        let root: Uigf = serde_json::from_str(DATA).unwrap();
        let serialized = serde_json::to_string(&root).unwrap();
        let roundtrip: Uigf = serde_json::from_str(&serialized).unwrap();

        let items = &roundtrip.hk4e.as_ref().unwrap()[0].list;
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].name.as_deref(), Some("弹弓"));
        assert_eq!(items[3].name.as_deref(), Some("班尼特"));
    }
}
