Goblin = {
    name = "Goblin",
    glyph = "g",
    armor = 5,
    max_health = 10,
    speed = 5,
    collidable = true,
    walk = {
        dx = 0,
        dy = 0
    }
}

GoblinShaman = {
    prototype = "Goblin",
    name = "Goblin Shaman",
    armor = 7,
    max_health = 20
}

register_entity("Goblin", Goblin)
register_entity("GoblinShaman", GoblinShaman)