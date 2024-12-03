-- Load the MongoDB module
require("mongo")
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
local database = client:Database("gmod_data")
local collection = database:Collection("players")

hook.Add("PlayerConnect", "SavePlayerData", function(name, address)


    local document = {
        name = name,
        address = address,
        money = 0,  -- Starter money
        level = 1   -- Starter level
    }

    local result = collection:Insert(document)
    if result then
        print("Player data saved for: " .. name)
    else
        print("Failed to save player data for: " .. name)
    end
end)