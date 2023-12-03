use anyhow::Context;
use rayon::prelude::*;

mod inventory_json;

fn main() -> anyhow::Result<()> {
    let inventory_json = std::env::args()
        .nth(1)
        .expect("missing path to inventory.json");

    let inventory: inventory_json::InventoryJsonRoot =
        serde_json::from_reader(std::fs::File::open(inventory_json)?)?;

    let entries = inventory.inventory;
    println!("loaded {} entries!", entries.len());

    let all_tradable = entries
        .into_iter()
        .filter(|x| matches!(x.tradeable, inventory_json::Tradeable::True))
        .collect::<Vec<_>>();

    println!("found {} tradable entries", all_tradable.len());

    let tradable = all_tradable
        .into_iter()
        .filter(|x| {
            x.blueprint_item.is_empty() && !matches!(x.slot, inventory_json::Slot::Blueprint)
        })
        .collect::<Vec<_>>();

    println!("found {} tradable entries (no blueprints)", tradable.len());

    let tradable = tradable
        .into_iter()
        .map(SimpleInventoryEntry::from_json)
        .collect::<Vec<_>>();

    // now, the web part
    let pc_price_selector = scraper::Selector::parse("#SteamPrice > .pfData").unwrap();

    let (tradable_with_prices, errors) = tradable
        .into_par_iter()
        .map(|entry| (fetch_price(&pc_price_selector, &entry), entry))
        .partition::<Vec<_>, Vec<_>, _>(|x| x.0.is_ok());

    let mut tradable_with_prices = tradable_with_prices
        .into_iter()
        .map(|(res, e)| (res.unwrap(), e))
        .collect::<Vec<_>>();

    let errors = errors
        .into_iter()
        .map(|(res, e)| (res.unwrap_err(), e))
        .collect::<Vec<_>>();

    tradable_with_prices.sort_by_key(|x| x.0 .0);

    let mut low_bound = 0;
    let mut high_bound = 0;
    for ((low, high), entry) in tradable_with_prices {
        println!("{},{},{}", low, high, entry.to_insider_gg_query(false));
        low_bound += low;
        high_bound += high;
    }

    println!("total inventory is worth {} - {}", low_bound, high_bound);

    for (error, entry) in errors {
        eprintln!("!{:?}: {:#?}", entry, error)
    }

    Ok(())
}

fn fetch_price(
    pc_price_selector: &scraper::Selector,
    entry: &SimpleInventoryEntry,
) -> anyhow::Result<(usize, usize)> {
    let url = entry.to_insider_gg_query(false);
    eprintln!("fetching {url}...");

    let raw_html = reqwest::blocking::get(url)
        .context("while fetching url")?
        .text()?;

    let document = scraper::Html::parse_document(&raw_html);

    let pc_price_raw = document
        .select(pc_price_selector)
        .map(|x| x.inner_html())
        .next()
        .context("could not find pc price on page")?;

    let (pc_price_low, pc_price_high) = pc_price_raw
        .trim()
        .split_once(" - ")
        .with_context(|| format!("malformed pc price: {}", pc_price_raw))?;

    let (pc_price_low, pc_price_high): (usize, usize) = (
        pc_price_low.parse().context(pc_price_raw.clone())?,
        pc_price_high.parse().context(pc_price_raw.clone())?,
    );

    Ok((pc_price_low, pc_price_high))
}

#[derive(Debug)]
pub struct SimpleInventoryEntry {
    pub name: String,
    pub slot: inventory_json::Slot,
    pub paint: inventory_json::Paint,
    pub certification: inventory_json::Certification,
    pub quality: inventory_json::Quality,
    pub amount: usize,
    pub special_edition: inventory_json::SpecialEdition,
}

impl SimpleInventoryEntry {
    fn from_json(entry: inventory_json::InventoryEntry) -> SimpleInventoryEntry {
        let inventory_json::InventoryEntry {
            product_id: _,
            name,
            slot,
            paint,
            certification,
            certification_value: _,
            rank_label: _,
            quality,
            amount,
            special_edition,
            blueprint_item_id: _,
            blueprint_item: _,
            blueprint_cost: _,
            tradeable: _,
        } = entry;

        SimpleInventoryEntry {
            name,
            slot,
            paint,
            certification,
            quality,
            amount,
            special_edition,
        }
    }

    fn to_insider_gg_query(&self, with_quality: bool) -> String {
        let slot = match self.slot {
            inventory_json::Slot::Unknown => unreachable!(),
            inventory_json::Slot::AnimatedDecal => "decals",
            inventory_json::Slot::Antenna => "antennas",
            inventory_json::Slot::AvatarBorder => "avatar_borders",
            inventory_json::Slot::Blueprint => unreachable!(),
            inventory_json::Slot::Body => "cars",
            inventory_json::Slot::Crate => "crates",
            inventory_json::Slot::Decal => "decals",
            inventory_json::Slot::EngineAudio => "engine_sounds",
            inventory_json::Slot::GoalExplosion => "goal_explosions",
            inventory_json::Slot::PaintFinish => "paint_finishes",
            inventory_json::Slot::PlayerAnthem => unreachable!(),
            inventory_json::Slot::PlayerBanner => "banners",
            inventory_json::Slot::PlayerTitle => unreachable!(),
            inventory_json::Slot::RewardItem => unreachable!(),
            inventory_json::Slot::RocketBoost => "boosts",
            inventory_json::Slot::Topper => "toppers",
            inventory_json::Slot::Trail => "trails",
            inventory_json::Slot::Wheels => "wheels",
        };

        // fixup
        let name = match self.name.as_str() {
            "Blade Wave" => "Blade Wave 2020: Inverted",
            s => s,
        };

        let name = name
            .to_lowercase()
            .replace('!', "") // "WWE SmackDown Live!"
            .replace(" - ", "_") // "Lunchbox - Esper"
            .replace(['.', '-', '\'', '&', '!'], "_") // "Y.O.U", "Tri-2050", "School'd", "Nuts & Bolts"
            .replace(": ", "/") // "Octane: Krush"
            .replace(' ', "_")
            .replace("__", "_") // "Golden Moon '23"
            .replace("___", "_"); // "Nuts & Bolts"
        let name = name.trim_end_matches('_');

        let paint = match self.paint {
            inventory_json::Paint::Black => Some("black"),
            inventory_json::Paint::BurntSienna => Some("sienna"),
            inventory_json::Paint::Cobalt => Some("cobalt"),
            inventory_json::Paint::Crimson => Some("crimson"),
            inventory_json::Paint::ForestGreen => Some("fgreen"),
            inventory_json::Paint::Grey => Some("grey"),
            inventory_json::Paint::Lime => Some("lime"),
            inventory_json::Paint::None => None,
            inventory_json::Paint::Orange => Some("orange"),
            inventory_json::Paint::Pink => Some("pink"),
            inventory_json::Paint::Purple => Some("purple"),
            inventory_json::Paint::Saffron => Some("saffron"),
            inventory_json::Paint::SkyBlue => Some("sblue"),
            inventory_json::Paint::TitaniumWhite => Some("white"),
        };

        let special_edition = match self.special_edition {
            inventory_json::SpecialEdition::None => None,
            inventory_json::SpecialEdition::Holographic => Some("holographic"),
            inventory_json::SpecialEdition::Infinite => Some("infinite"),
            inventory_json::SpecialEdition::Inverted => Some("inverted"),
            inventory_json::SpecialEdition::Remixed => Some("remixed"),
        };

        let quality = if with_quality {
            match self.quality {
                inventory_json::Quality::None => None,
                inventory_json::Quality::BlackMarket => Some("black_market"),
                inventory_json::Quality::Common => Some("common"),
                inventory_json::Quality::Exotic => Some("exotic"),
                inventory_json::Quality::Import => Some("import"),
                inventory_json::Quality::Legacy => Some("legacy"),
                inventory_json::Quality::Limited => Some("limited"),
                inventory_json::Quality::Rare => Some("rare"),
                inventory_json::Quality::Uncommon => Some("uncommon"),
                inventory_json::Quality::VeryRare => Some("very_rare"),
            }
        } else {
            None
        };

        let maybe_present = |x: Option<&str>| x.map(|s| format!("/{s}")).unwrap_or_default();

        format!(
            "https://rl.insider.gg/en/pc/{slot}/{name}{}{}{}",
            maybe_present(quality),
            maybe_present(special_edition),
            maybe_present(paint),
        )
    }
}
