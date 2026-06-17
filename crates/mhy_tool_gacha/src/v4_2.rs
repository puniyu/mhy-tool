use mhy_genshin_gacha::v4_2::Uigf as GenshinRoot;

/// 游戏抽卡记录包装
#[derive(Debug, Clone)]
pub enum Game<'a> {
    Genshin(GenshinRoot<'a>),
}
