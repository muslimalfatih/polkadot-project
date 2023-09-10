// Define a struct called FilterCondition with a single 
// field of the desired type for filtering.
struct FilterCondition {
    filter_value: i32,
}

// Implement a method called is_match on the FilterCondition struct that takes a reference 
// to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.filter_value
    }
}

// Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments.
// The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection = Vec::new();
    for item in collection {
        if condition.is_match(item) {
            filtered_collection.push(*item);
        }
    }
    filtered_collection
}

fn main() {
    // Create a collection (e.g., a vector) with some elements.
    let collection = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { filter_value: 3 };
    
    // Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
    let filtered_collection = custom_filter(&collection, &filter_condition);

    // Print the filtered result to the console.
    println!("{:?}", filtered_collection);
}
