# minigrep

This is a command-line tool built with Rust that searches for a specified string pattern in a file and returns the lines containing that pattern. The tool is built as part of the Rust Programming Language Book [2nd Edition].

##Usage
You can run the program using the following command:

``cargo run {query} {filename}``

Where:
query is the string pattern you are looking for
filename is the path to the file you want to search in
The program will then print the lines in the file that contain the query string.

##Code Structure
The main functionality of the tool is implemented in src/lib.rs. The Config struct defined in src/lib.rs holds the configuration options provided by the user, while the run function in src/lib.rs implements the search functionality.

The main function in src/main.rs is responsible for parsing the command-line arguments, building the Config object, and running the search. If any errors occur during the process, the program will print an error message and exit with a non-zero status code.

#Dependencies
The tool uses the following Rust crates:

`std::env`: provides access to command-line arguments
`std::process`: provides functions for interacting with the process environment

#MiniGrep Library
This library provides functionality for searching for a string pattern in a file.

##Config Struct
The Config struct holds the configuration options provided by the user. It has the following fields:

`query`: a String representing the string pattern to search for
`file_path`: a String representing the path to the file to search in
`case_sensitive`: a bool indicating whether the search should be case-sensitive or not

##Functions

``pub fn build<'a>(args: &'a [String]) -> Result<Config, &'static str>``
This function takes a reference to a vector of String arguments and returns a Result containing a Config object or a static string error message. It parses the arguments and builds the Config object. If there are not enough arguments, an error message is returned.

``pub fn run(&self) -> Result<(), Box<dyn Error>>``
This function takes a reference to a Config object and returns a Result containing either an empty tuple or a boxed dynamic error. It runs the search and prints the lines in the file that contain the query string. If there is an error reading the file, it is returned as a boxed dynamic error.

``pub fn search_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>``
This function takes a string query and the contents of a file, and returns a vector of string slices representing the lines in the file that contain the query string, in a case-sensitive manner.

``pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>``
This function takes a string query and the contents of a file, and returns a vector of string slices representing the lines in the file that contain the query string, in a case-insensitive manner.

##Tests
The tests module contains unit tests for the search_sensitive and search functions. These tests ensure that the functions return the expected results when searching for a query string in a given text.
