local script_name = "dc_awesome_ring"

function on_equip()
    player.add_item("WeapSteelLongsword", 1)
    player.equip_item("WeapSteelLongsword", 1)
end

function on_unequip() {
    player.unequip_item("WeapSteelLongsword", 1)
    player.remove_item("WeapSteelLongsword", 1)
}