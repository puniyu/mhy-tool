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
