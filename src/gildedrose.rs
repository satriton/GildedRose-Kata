use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
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

const MAX_QUALITY: i32 = 50;
pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        let aged_brie: String = "Aged Brie".to_string();
        let backstage: String = "Backstage passes to a TAFKAL80ETC concert".to_string();
        let sulfuras: String = "Sulfuras, Hand of Ragnaros".to_string();

        for item in self.items.iter_mut() {
            if item.name == sulfuras {
                continue;
            }

            item.sell_in -= 1;

            Self::update_item(item, &aged_brie, &backstage);

            if item.name.contains("Conjured") {
                Self::update_item(item, &aged_brie, &backstage);
            }

            if item.sell_in < 0 {
                Self::handle_expired_item(item, &aged_brie, &backstage);
            }
        }
    }

    fn update_item(item: &mut Item, aged_brie: &String, backstage: &String) {
        if item.name.contains(aged_brie) {
            Self::update_aged_brie(item);
        } else if item.name.contains(backstage) {
            Self::update_backstage(item);
        } else {
            Self::decrease_quality(item);
        }
    }

    fn update_backstage(item: &mut Item) {
        Self::increase_quality(item);

        if item.sell_in < 10 {
            Self::increase_quality(item);
        }

        if item.sell_in < 5 {
            Self::increase_quality(item);
        }
    }

    fn handle_expired_item(item: &mut Item, aged_brie: &String, backstage: &String) {
        if item.name.contains(aged_brie) {
            Self::update_aged_brie(item);
        } else if item.name.contains(backstage) {
            item.quality = 0;
        } else {
            Self::decrease_quality(item);
        }
    }

    fn update_aged_brie(item: &mut Item) {
        Self::increase_quality(item);
    }

    fn decrease_quality(item: &mut Item) {
        if item.quality > 0 {
            item.quality -= 1;
        }
    }

    fn increase_quality(item: &mut Item) {
        if item.quality < MAX_QUALITY {
            item.quality += 1;
        }
    }
}

pub struct GildedRoseGolden {
    pub items: Vec<Item>,
}

impl GildedRoseGolden {
    pub fn new(items: Vec<Item>) -> GildedRoseGolden {
        GildedRoseGolden { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if self.items[i].name != "Aged Brie"
                && self.items[i].name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != "Aged Brie" {
                    if self.items[i].name != "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].quality > 0 {
                            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, GildedRoseGolden, Item};

    #[test]
    pub fn should_behave_like_golden_master() {
        let items = vec![
            Item::new("+5 Dexterity Vest", 10, 20),
            Item::new("Aged Brie", 2, 0),
            Item::new("Elixir of the Mongoose", 5, 7),
            Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
            Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
        ];
        let mut rose = GildedRose::new(items.clone());
        let mut golden_rose = GildedRoseGolden::new(items);

        for _i in 1..100 {
            rose.update_quality();
            golden_rose.update_quality();

            assert_eq!(golden_rose.items, rose.items);
        }
    }

    #[test]
    pub fn conjured_item_quality_should_decrese_two_time_more() {
        let items = vec![
            Item::new("Conjured Mana Cake", 3, 6),
            Item::new("Mana Cake", 3, 6),
            Item::new("Conjured Aged Brie", 2, 0),
            Item::new("Aged Brie", 2, 0),
        ];

        let mut rose = GildedRose::new(items.clone());

        rose.update_quality();

        assert_eq!(rose.items[0].quality, 4);
        assert_eq!(rose.items[1].quality, 5);
        assert_eq!(rose.items[2].quality, 2);
        assert_eq!(rose.items[3].quality, 1);
    }
}
