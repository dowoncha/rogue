enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary
}

enum ItemType {
    Armor,
    Weapon,
    Potion,
    Ring,
    Weapon
}

struct Item {
    name: String,
    i_type: ItemType,
    rarity: Rarity,
    description: String
}

// enum WeaponSpee

enum DamageType {
    Pierce,
    Slash,
    Crush
}

enum Handedness {
    One,
    Two
}

struct Weapon {
    item: Item,
    min_base_damage: u32,
    max_base_damage: u32,
    speed: u16,
    interrupt: f32,
    range: u16,
    damage_type: DamageType,
    handedness: Handedness
}
