-- Connect to the MongoDB client
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
-- Select the database
local database = client:Database("testdb")

-- Create a new collection named "example_collection"
local result = database:CreateCollection("example_collection")
if result then
    print("Collection 'example_collection' created successfully")
else
    print("Failed to create collection")
end