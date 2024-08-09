# rust_tantivy

`rust_tantivy` is a Rust-based project utilizing the Tantivy search engine library to index and search the contents of files within a specified folder. The project supports searching for single terms, phrases, and regular expressions, with capabilities to update the index whenever files are modified.

## Table of Contents

-   [Installation](#installation)
-   [Usage](#usage)
-   [Project Structure](#project-structure)
-   [Functionality](#functionality)
    -   [Index Creation](#index-creation)
    -   [Index Update](#index-update)
    -   [Search](#search)
    -   [Reports](#reports)
-   [Examples](#examples)
-   [Dependencies](#dependencies)
-   [License](#license)

## Installation

To get started with `rust_tantivy`, ensure that Rust is installed on your system. If Rust is not installed, you can install it by following the instructions here.

1.  Clone the repository:
    ```bash
    git clone https://github.com/itsmesamster/rust_tantivy.git
    cd rust_tantivy
2.  Build the project
    ```bash
    cargo build --release`
## Usage

To use `rust_tantivy`, specify the folder you want to index and search within. The following command demonstrates how to run the main program:
```bash
cargo run --release -- <folder_path>`
```
Replace `<folder_path>` with the path to the folder you want to index.

### Example Command:
```bash
cargo run --release -- /path/to/your/folder`
```
This command indexes the folder's contents and performs searches for predefined terms, phrases, and regex patterns.

## Project Structure
-   **Cargo.toml**: The manifest file for Rust, containing metadata for the project and its dependencies.
-   **src/main.rs**: The program's entry point, handling command-line arguments and invoking indexing and searching functions.
-   **src/lib.rs**: Contains the core functionality for indexing, updating the index, searching, and generating reports.

## Functionality

### Index Creation

The `create_index_from_folder` function indexes all files in the specified folder. It reads the content of each file and indexes it based on the schema defined within the function. The index is stored in the directory specified by `index_path`.

### Index Update

The `update_index_with_new_files` function updates the existing index by identifying files added or modified since the last update. It also removes files from the index that are no longer present in the folder.

### Search

The project supports searching for:

-   **Single Terms**: Individual words or tokens.
-   **Phrases**: Sequences of words enclosed in quotes.
-   **Regular Expressions**: Patterns that match specific character sequences.

These searches are handled by the `search_terms_in_index`, `search_phrases_in_index`, and `search_regex_in_index` functions, respectively.

### Reports

The `create_report` function generates an HTML report of the search results, listing the files where terms, phrases, or regex patterns were found.

## Examples

### Search Terms
To search for single terms in the indexed files:
```bash
let search_terms: Vec<&str> = vec!["Australia"];`
```
### Search Phrases
To search for specific phrases:
```bash
let search_phrases: Vec<&str> = vec!["Cross Roads, Ripley County, Indiana"];`
```
### Search Regular Expressions
To search using regular expressions:
```bash
let search_regex: Vec<&str> = vec!["d[ai]{2}ry"];`
```
## Dependencies
The project relies on the following Rust crates:
-   tantivy - A full-text search engine library in Rust.
-   regex - A regular expression library for Rust.
-   log - A logging facade for Rust.
-   walkdir - A Rust library for recursive directory traversal.
-   htmlescape - A library for escaping HTML entities in Rust.
## License
This project is licensed under the MIT License. See the LICENSE file for more details.