-- Load the MongoDB module
require("mongo")
-- Connect to the MongoDB client
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
-- Select the database
local database = client:Database("testdb")
-- Select the collection
local collection = database:Collection("example_collection")

-- Prepare filter for update
local filter = {
    name = "John Doe"
}

-- Prepare update document (change age)
local update = {
    ['$set'] = {
        age = 35
    }
}

-- Update documents based on the filter
local result = collection:Update(filter, update)
if result then
    print("Documents updated successfully")
else
    print("Failed to update documents")
end
