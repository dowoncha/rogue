Zombie = {
    name = "zombie",
    glyph = 'z',
    armor = 3,
    speed = 5,
    max_health = 10
}

function Zombie:update() {

}

function Zombie:onTurn()
    dx = math.random(-1, 1)
    dy = math.random(-1, 1)

    self:move(dx, dy)
end