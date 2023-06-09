# rl-inventory-prices

A quick-and-dirty script that iterates over every item in a user's Rocket League
inventory, and then fetches the current trade-price of each item from
`rl.insider.gg`.

## Context

New rocket league season came out, and I was ~200 credits shy of picking up the
new Rocket Pass. Instead of randomly guessing what items I have lying around
that might be worth something, I thought it'd be easier (?) to write a script to
figure that out for me.

And yeah, all things considered - it really wasn't that hard!

Sure, there are still some bugs to weed out (i.e: some item URLs don't get
constructed correctly)... but for a couple hours on a Thursday night, I consider
this a great success!

## Performance

Downloads items info in parallel (using threads, via `rayon`), so it's
reasonably fast.

Takes ~10 secs to download ~1000 items (on my 12-core machine, with a solid
internet connection).

Lots of room for improvement here of course, but again - totally fine for a few
hours of weeknight hacking.

## Usage

- Install [BakkesMod](https://bakkesplugins.com/)
- Install the [Better Inventory Export](https://bakkesplugins.com/plugins/view/155) plugin
- Follow the instructions on the plugin page to dump your inventory as an `inventory.json`
- Clone this repo
- Run `cargo run -- path/to/inventory.json`
- Be patient (it'd downloading stuff...)
- ???
- Profit!

Output should look something along the lines of:

```
$ cargo run -- ./inventory.json
   Compiling rl-inventory-prices v0.1.0 (/home/daprilik/src/rl-inventory-prices)
    Finished dev [unoptimized + debuginfo] target(s) in 1.65s
     Running `target/debug/rl-inventory-prices ./inventory.json`
loaded 4740 entries!
found 2141 tradable entries
found 1163 tradable entries (no blueprints)
fetching https://rl.insider.gg/en/pc/trails/ruckus...
fetching https://rl.insider.gg/en/pc/decals/liquid_camo/white...
fetching https://rl.insider.gg/en/pc/wheels/cephalo/fgreen...
...
10,20,https://rl.insider.gg/en/pc/antennas/thermometer
10,20,https://rl.insider.gg/en/pc/toppers/lunchbox_esper
10,20,https://rl.insider.gg/en/pc/banners/yolkel
20,40,https://rl.insider.gg/en/pc/banners/tread_heavily
...
total inventory is worth 1000 - 2000
!SimpleInventoryEntry { name: "Nomad: Blinkpad", slot: AnimatedDecal, paint: ForestGreen, certification: None, quality: Limited, amount: 1, special_edition: None }: could not find pc price on page
!SimpleInventoryEntry { name: "Stern", slot: Wheels, paint: Cobalt, certification: None, quality: None, amount: 1, special_edition: None }: could not find pc price on page
!SimpleInventoryEntry { name: "OEM", slot: Wheels, paint: Orange, certification: None, quality: None, amount: 1, special_edition: None }: could not find pc price on page
```
