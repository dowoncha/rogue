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
    rarity: Rarity,
    description: String
    attack: Attack,
    ranged: Attack,
    defense: Defense,
    _use: Box<Use>
}

struct Attack {
    min_damange: i32,
    max_damage: i32
}

impl Attack {
    pub fn hit() {
    }
}

trait Use {
    fn use();
}

struct HealUse {

}

impl Use for HealUse {

}

struct Defense {

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
