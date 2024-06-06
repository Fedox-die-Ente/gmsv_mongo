-- Connect to the MongoDB client
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
-- Select the database
local database = client:Database("testdb")
-- Select the collection
local collection = database:Collection("example_collection")

-- Prepare filter for deletion
local filter = {
    name = "John Doe"
}

-- Delete documents based on the filter
local result = collection:Delete(filter)
if result then
    print("Documents deleted successfully")
else
    print("Failed to delete documents")
end
