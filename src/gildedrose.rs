use std::fmt::{self, Display};

const MIN_ITEM_QUALITY: i32 = 0;
const MAX_ITEM_QUALITY: i32 = 50;

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in self.items.iter_mut() {
            let is_legendary = item.name == "Sulfuras, Hand of Ragnaros";
            if is_legendary {
                continue;
            }

            let is_pass = item.name.starts_with("Backstage passes to");
            if is_pass {
                item.quality = match item.sell_in {
                    6..=10 => item.quality + 2,
                    1..=5 => item.quality + 3,
                    0 => 0,
                    _ => item.quality + 1,
                };
                item.sell_in -= 1;
                continue;
            }

            let is_aged = item.name == "Aged Brie";
            if is_aged {
                let new_quality = item.quality + if item.sell_in == 0 { 2 } else { 1 };
                item.quality = i32::min(new_quality, MAX_ITEM_QUALITY);
                item.sell_in -= 1;
                continue;
            }

            let is_conjured = item.name.starts_with("Conjured");
            let regular_quality_loss_amount = if item.sell_in == 0 { 2 } else { 1 };
            let new_quality =
                item.quality - regular_quality_loss_amount * if is_conjured { 2 } else { 1 };
            item.quality = i32::max(new_quality, MIN_ITEM_QUALITY);
            item.sell_in -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn regular_item() {
        let name = "foo";
        let sell_in = 1;
        let quality = 3;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(sell_in - 1, rose.items[0].sell_in);
        assert_eq!(quality - 1, rose.items[0].quality);

        rose.update_quality();
        assert_eq!(quality - 3, rose.items[0].quality);

        rose.update_quality();
        assert!(rose.items[0].quality >= MIN_ITEM_QUALITY);
    }

    #[test]
    pub fn aged_item() {
        let name = "Aged Brie";
        let sell_in = 1;
        let quality = 47;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(quality + 1, rose.items[0].quality);
        assert_eq!(sell_in - 1, rose.items[0].sell_in);

        rose.update_quality();
        assert_eq!(quality + 3, rose.items[0].quality);
        assert!(rose.items[0].quality <= MAX_ITEM_QUALITY);
    }

    #[test]
    pub fn legendary_item() {
        let name = "Sulfuras, Hand of Ragnaros";
        let sell_in = 5;
        let quality = 80;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(sell_in, rose.items[0].sell_in);
        assert_eq!(quality, rose.items[0].quality);
    }

    #[test]
    pub fn pass_item() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let sell_in_10_or_less = 10;
        let sell_in_5_or_less = 5;
        let sell_in_0 = 0;
        let sell_in_more_than_10 = 20;
        let quality = 10;
        let items = vec![
            Item::new(name, sell_in_10_or_less, quality),
            Item::new(name, sell_in_5_or_less, quality),
            Item::new(name, sell_in_0, quality),
            Item::new(name, sell_in_more_than_10, quality),
        ];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(sell_in_10_or_less - 1, rose.items[0].sell_in);
        assert_eq!(quality + 2, rose.items[0].quality);
        assert_eq!(quality + 3, rose.items[1].quality);
        assert_eq!(0, rose.items[2].quality);
        assert_eq!(quality + 1, rose.items[3].quality);
    }

    #[test]
    pub fn conjured_item() {
        let name = "Conjured Mana Cake";
        let sell_in = 1;
        let quality = 7;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(sell_in - 1, rose.items[0].sell_in);
        assert_eq!(quality - 2, rose.items[0].quality);

        rose.update_quality();
        assert_eq!(quality - 6, rose.items[0].quality);

        rose.update_quality();
        assert!(rose.items[0].quality >= MIN_ITEM_QUALITY);
    }
}
