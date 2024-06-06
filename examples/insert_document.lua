-- Connect to the MongoDB client
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
-- Select the database
local database = client:Database("testdb")
-- Select the collection
local collection = database:Collection("example_collection")

-- Prepare document to insert
local document = {
    name = "John Doe",
    age = 30,
    city = "New York"
}

-- Insert document into the collection
local result = collection:Insert(document)
if result then
    print("Document inserted successfully")
else
    print("Failed to insert document")
end
