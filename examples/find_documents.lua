-- Load the MongoDB module
require("mongo")
-- Connect to the MongoDB client
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
-- Select the database
local database = client:Database("testdb")
-- Select the collection
local collection = database:Collection("example_collection")

-- Prepare filter for query
local filter = {
    name = "John Doe"
}

-- Find documents based on the filter
local documents = collection:Find(filter)
-- Iterate through each found document and print
for i, doc in ipairs(documents) do
    print("Document " .. i .. ":")
    print("Name: " .. doc.name)
    print("Age: " .. doc.age)
    print("City: " .. doc.city)
end
