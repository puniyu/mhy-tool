use std::borrow::Cow;

use mhy_tool_common::{
    Uin,
    gacha::{Lang, UigfGachaType, datetime_serde},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hk4eGachaType {
    #[serde(rename = "100")]
    A100,
    #[serde(rename = "200")]
    A200,
    #[serde(rename = "301")]
    A301,
    #[serde(rename = "302")]
    A302,
    #[serde(rename = "400")]
    A400,
    #[serde(rename = "500")]
    A500,
}

impl From<Hk4eGachaType> for UigfGachaType {
    fn from(gacha_type: Hk4eGachaType) -> Self {
        match gacha_type {
            Hk4eGachaType::A100 => UigfGachaType::A100,
            Hk4eGachaType::A200 => UigfGachaType::A200,
            Hk4eGachaType::A301 | Hk4eGachaType::A400 => UigfGachaType::A301,
            Hk4eGachaType::A302 => UigfGachaType::A302,
            Hk4eGachaType::A500 => UigfGachaType::A500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player<'a> {
    pub uid: Uin,
    pub timezone: i32,
    pub lang: Option<Lang>,
    #[serde(borrow)]
    pub list: Vec<Item<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item<'a> {
    pub uigf_gacha_type: UigfGachaType,
    pub gacha_type: Hk4eGachaType,
    #[serde(borrow)]
    pub item_id: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub count: Option<Cow<'a, str>>,
    #[serde(with = "datetime_serde")]
    pub time: jiff::civil::DateTime,
    #[serde(borrow)]
    pub name: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub item_type: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub rank_type: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub id: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mhy_tool_common::gacha::UigfGachaType;

    #[test]
    fn test_hk4e_to_uigf_gacha_type_mapping() {
        assert_eq!(UigfGachaType::from(Hk4eGachaType::A100), UigfGachaType::A100);
        assert_eq!(UigfGachaType::from(Hk4eGachaType::A200), UigfGachaType::A200);
        assert_eq!(UigfGachaType::from(Hk4eGachaType::A301), UigfGachaType::A301);
        assert_eq!(UigfGachaType::from(Hk4eGachaType::A302), UigfGachaType::A302);
        // A400 maps to A301 (merged)
        assert_eq!(UigfGachaType::from(Hk4eGachaType::A400), UigfGachaType::A301);
        assert_eq!(UigfGachaType::from(Hk4eGachaType::A500), UigfGachaType::A500);
    }

    #[test]
    fn test_item_deserialize() {
        let json = r#"{
            "uigf_gacha_type": "301",
            "gacha_type": "301",
            "item_id": "10000032",
            "count": "1",
            "time": "2022-11-02 10:49:39",
            "name": "班尼特",
            "item_type": "武器",
            "rank_type": "4",
            "id": "1667354760051767691"
        }"#;

        let item: Item<'_> = serde_json::from_str(json).unwrap();
        assert_eq!(item.gacha_type, Hk4eGachaType::A301);
        assert_eq!(item.uigf_gacha_type, UigfGachaType::A301);
        assert_eq!(item.name.as_deref(), Some("班尼特"));
        assert_eq!(item.item_id.as_deref(), Some("10000032"));
        assert_eq!(item.rank_type.as_deref(), Some("4"));
        assert_eq!(item.id.as_ref(), "1667354760051767691");
    }

    #[test]
    fn test_item_serialize_roundtrip() {
        let json = r#"{
            "uigf_gacha_type": "301",
            "gacha_type": "301",
            "item_id": "10000032",
            "count": "1",
            "time": "2022-11-02 10:49:39",
            "name": "班尼特",
            "item_type": "武器",
            "rank_type": "4",
            "id": "1667354760051767691"
        }"#;

        let item: Item<'_> = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&item).unwrap();
        let roundtrip: Item<'_> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(roundtrip.gacha_type, Hk4eGachaType::A301);
        assert_eq!(roundtrip.name.as_deref(), Some("班尼特"));
        assert_eq!(roundtrip.id.as_ref(), "1667354760051767691");
    }
}
