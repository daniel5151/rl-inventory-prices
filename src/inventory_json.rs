use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Slot {
    #[serde(rename = "Animated Decal")]
    AnimatedDecal,
    #[serde(rename = "Antenna")]
    Antenna,
    #[serde(rename = "Avatar Border")]
    AvatarBorder,
    #[serde(rename = "Blueprint")]
    Blueprint,
    #[serde(rename = "Body")]
    Body,
    #[serde(rename = "Crate")]
    Crate,
    #[serde(rename = "Decal")]
    Decal,
    #[serde(rename = "Engine Audio")]
    EngineAudio,
    #[serde(rename = "Goal Explosion")]
    GoalExplosion,
    #[serde(rename = "Paint Finish")]
    PaintFinish,
    #[serde(rename = "Player Anthem")]
    PlayerAnthem,
    #[serde(rename = "Player Banner")]
    PlayerBanner,
    #[serde(rename = "Player Title")]
    PlayerTitle,
    #[serde(rename = "Reward Item")]
    RewardItem,
    #[serde(rename = "Rocket Boost")]
    RocketBoost,
    #[serde(rename = "Topper")]
    Topper,
    #[serde(rename = "Trail")]
    Trail,
    #[serde(rename = "Wheels")]
    Wheels,
    #[serde(rename = "")]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Paint {
    #[serde(rename = "Black")]
    Black,
    #[serde(rename = "Burnt Sienna")]
    BurntSienna,
    #[serde(rename = "Cobalt")]
    Cobalt,
    #[serde(rename = "Crimson")]
    Crimson,
    #[serde(rename = "Forest Green")]
    ForestGreen,
    #[serde(rename = "Grey")]
    Grey,
    #[serde(rename = "Lime")]
    Lime,
    #[serde(rename = "none", alias = "")]
    None,
    #[serde(rename = "Orange")]
    Orange,
    #[serde(rename = "Pink")]
    Pink,
    #[serde(rename = "Purple")]
    Purple,
    #[serde(rename = "Saffron")]
    Saffron,
    #[serde(rename = "Sky Blue")]
    SkyBlue,
    #[serde(rename = "Titanium White")]
    TitaniumWhite,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Certification {
    #[serde(rename = "AerialGoals")]
    AerialGoals,
    #[serde(rename = "Assists")]
    Assists,
    #[serde(rename = "BackwardsGoals")]
    BackwardsGoals,
    #[serde(rename = "BicycleGoals")]
    BicycleGoals,
    #[serde(rename = "Centers")]
    Centers,
    #[serde(rename = "Clears")]
    Clears,
    #[serde(rename = "EpicSaves")]
    EpicSaves,
    #[serde(rename = "Goals")]
    Goals,
    #[serde(rename = "Juggles")]
    Juggles,
    #[serde(rename = "LongGoals")]
    LongGoals,
    #[serde(rename = "MVPs")]
    MVPs,
    #[serde(rename = "none", alias = "")]
    None,
    #[serde(rename = "Saves")]
    Saves,
    #[serde(rename = "ShotsOnGoal")]
    ShotsOnGoal,
    #[serde(rename = "TurtleGoals")]
    TurtleGoals,
    #[serde(rename = "Wins")]
    Wins,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Quality {
    #[serde(rename = "", alias = "unknown")]
    None,
    #[serde(rename = "Black market")]
    BlackMarket,
    #[serde(rename = "Common")]
    Common,
    #[serde(rename = "Exotic")]
    Exotic,
    #[serde(rename = "Import")]
    Import,
    #[serde(rename = "Legacy")]
    Legacy,
    #[serde(rename = "Limited")]
    Limited,
    #[serde(rename = "Rare")]
    Rare,
    #[serde(rename = "Uncommon")]
    Uncommon,
    #[serde(rename = "Very rare")]
    VeryRare,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialEdition {
    #[serde(rename = "none", alias = "")]
    None,
    #[serde(rename = "Edition_Holographic")]
    Holographic,
    #[serde(rename = "Edition_Infinite")]
    Infinite,
    #[serde(rename = "Edition_Inverted")]
    Inverted,
    #[serde(rename = "Edition_Remixed")]
    Remixed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Tradeable {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryEntry {
    pub product_id: isize,
    pub name: String,
    pub slot: Slot,
    pub paint: Paint,
    pub certification: Certification,
    pub certification_value: usize,
    pub rank_label: String,
    pub quality: Quality,
    pub amount: usize,
    // pub crate: String, // deprecated
    // pub instance_id: usize, // deprecated
    pub special_edition: SpecialEdition,
    pub blueprint_item_id: usize,
    pub blueprint_item: String,
    pub blueprint_cost: usize,
    pub tradeable: Tradeable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryJsonRoot {
    pub inventory: Vec<InventoryEntry>,
}
