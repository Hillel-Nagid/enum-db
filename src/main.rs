// mod builder;
mod inititator;
fn main() {
    inititator::initialize("./test.csv", "./types.txt"); //?
                                                         // builder::build();
    println!("Hello, world!");
}
// CLI tool that gets a csv file name as a parameter
// Commands:
//  init <filename> (csv file with data)
//  add <filename> (csv file with additional rows)
//  build (generates an output directory, and in it creates a .EDB file, and a binary file)
// Get columns' names AND TYPES from user
// Get data as csv file
// Caluculate min vec length needed for each column
// Generate a file with enums for each column with unique variants
// Generate a .EDB file with binary data
// Generate a retrival system with specified row buffer size, and columns' bufer size
