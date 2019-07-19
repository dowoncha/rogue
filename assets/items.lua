function FuncNew(obj) {
    function obj:new( o ) 
        o = o or {}
        setmetatable(0, self)
        self.__index = self
        return o
    end
    return obj
}

Item = {}
function Item:create()
    return FuncNew(Item):new()
end

Sword = Item:create()
Sword.name = "Sword"
function Item:create()
    return FuncNew(Sword):new()
end

ItemFactory = {}
ItemFactory.registryTable = {}
function ItemFactory:create()
    return ItemFactory
end

function ItemFactory:instantiate( name ) 
    for k,v in pairs(self.registryTable) do
        if v.name == name then
            return v.create()
        end
    end
    return nil
end

function ItemFactory:registry( item ) 
    print("Registering item '" .. item.name .. "'")
    table.insert(self.registryTable, item)
end

ItemFactory:registry(Sword)