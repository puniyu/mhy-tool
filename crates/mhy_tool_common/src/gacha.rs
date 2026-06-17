pub mod v4_0;
pub mod v4_1;
pub mod v4_2;

pub mod datetime_serde {
    use jiff::civil::DateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FMT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.strftime(FMT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        DateTime::strptime(FMT, s).map_err(serde::de::Error::custom)
    }
}

pub mod option_datetime_serde {
    use jiff::civil::DateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FMT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(dt) => serializer.serialize_str(&dt.strftime(FMT).to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<&str>::deserialize(deserializer)?;
        match s {
            Some(s) => DateTime::strptime(FMT, s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Lang {
    #[serde(rename = "de-de")]
    DeDe,
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "es-es")]
    EsEs,
    #[serde(rename = "fr-fr")]
    FrFr,
    #[serde(rename = "id-id")]
    IdId,
    #[serde(rename = "it-it")]
    ItIt,
    #[serde(rename = "ja-jp")]
    JaJp,
    #[serde(rename = "ko-kr")]
    KoKr,
    #[serde(rename = "pt-pt")]
    PtPt,
    #[serde(rename = "ru-ru")]
    RuRu,
    #[serde(rename = "th-th")]
    ThTh,
    #[serde(rename = "tr-tr")]
    TrTr,
    #[serde(rename = "vi-vn")]
    ViVn,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "zh-tw")]
    ZhTw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportTimestamp<'a> {
    #[serde(borrow)]
    String(Cow<'a, str>),
    Integer(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UigfGachaType {
    #[serde(rename = "100")]
    A100,
    #[serde(rename = "200")]
    A200,
    #[serde(rename = "301")]
    A301,
    #[serde(rename = "302")]
    A302,
    #[serde(rename = "500")]
    A500,
}
