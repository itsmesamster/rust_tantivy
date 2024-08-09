// use std::env;
use tantivy::schema::*;
use tantivy::{doc, Index};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::ReloadPolicy;
use walkdir::WalkDir;
use std::path::Path;
use std::fs::{self, File};
use std::time::Instant;
use std::io::Write;
use std::fs::metadata;
use tantivy::query::RegexQuery;
use tantivy::schema::{Schema, IndexRecordOption};
use std::time::SystemTime;
use std::collections::HashSet;
use tantivy::SegmentReader;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum StringType {
    Regex,
    Phrase,
    SingleTerm,
}

fn check_string_type(input: &str) -> StringType {
    let regex_special_chars = Regex::new(r"[.*+?^${}()|\[\]\\]").unwrap();

    if regex_special_chars.is_match(input) {
        return StringType::Regex;
    } else if input.contains(' ') {
        return StringType::Phrase;
    } else {
        return StringType::SingleTerm;
    }
}

// Function to update the index with new files added after the last index modification time
pub fn update_index_with_new_files(index_path: &str, folder_path: &str, last_index_time: SystemTime) -> tantivy::Result<()> {
    let index = Index::open_in_dir(index_path)?;
    let schema = index.schema();
    let path_field = schema.get_field("path").unwrap();
    let contents_field = schema.get_field("contents").unwrap();
    let mut index_writer = index.writer(15_000_000)?;

    let start_time = Instant::now();

    // Create a set to track the current files in the folder
    let mut current_files = HashSet::new();

    // Recursively read files from the folder and add new/modified files to the index
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let path_str = path.display().to_string();
            current_files.insert(path_str.clone());

            let metadata = metadata(path)?;
            let modified_time = metadata.modified()?;

            // Check if the file was modified after the last index time
            if modified_time > last_index_time {
                if let Ok(contents) = fs::read_to_string(path) {
                    // Optionally, limit the size of the contents
                    let limited_contents = if contents.len() > 10_000_000 {
                        &contents[..10_000_000]
                    } else {
                        &contents
                    };
                    let _ = index_writer.add_document(doc!(
                        path_field => path_str.clone(),
                        contents_field => limited_contents,
                    ));
                    println!("Added new or modified file to index: {}", path_str);
                }
            }
        }
    }

    // Fetch the paths of all indexed documents
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![path_field]);
    let query = query_parser.parse_query("*")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10_000))?;

    // Check for deletions
    for (_score, doc_address) in top_docs {
        let retrieved_doc : TantivyDocument = searcher.doc(doc_address)?;
        if let Some(path_value) = retrieved_doc.get_first(path_field) {
            if let tantivy::schema::OwnedValue::Str(path_str) = path_value {
                if !current_files.contains(path_str) {
                    println!("Path: {:?}", path_str);
                    // Delete the document from the index if the file is no longer present
                    index_writer.delete_term(Term::from_field_text(path_field, path_str));
                    index_writer.delete_term(Term::from_field_text(contents_field, path_str));
                    println!("Deleted file from index: {}", path_str);
                    // let _ = index_writer.delete_all_documents().unwrap();
                    // index_writer.commit()?;
                    // let _ = update_index_with_new_files(index_path, folder_path, last_index_time);
                }
            }
        }
    }


    // Commit the index
    index_writer.commit()?;
    // let _ = reader.reload();
    println!("Time taken to update index: {:?}", start_time.elapsed());

    Ok(())
}


// Function to update the index with new files added after the last index modification time
// pub fn update_index_with_new_files(index_path: &str, folder_path: &str, last_index_time: SystemTime) -> tantivy::Result<()> {
//     let index = Index::open_in_dir(index_path)?;
//     let schema = index.schema();
//     let path_field = schema.get_field("path").unwrap();
//     let contents_field = schema.get_field("contents").unwrap();
//     let mut index_writer = index.writer(15_000_000)?;

//     let start_time = Instant::now();
//     // Recursively read files from the folder
//     for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
//         if entry.file_type().is_file() {
//             let path = entry.path();
//             let metadata = metadata(path)?;
//             let modified_time = metadata.modified()?;

//             // Check if the file was modified after the last index time
//             if modified_time > last_index_time {
//                 let path_str = path.display().to_string();
//                 if let Ok(contents) = fs::read_to_string(path) {
//                     // Optionally, limit the size of the contents
//                     let limited_contents = if contents.len() > 10_000_000 {
//                         &contents[..10_000_000]
//                     } else {
//                         &contents
//                     };
//                     let _ = index_writer.add_document(doc!(
//                         path_field => path_str.clone(),
//                         contents_field => limited_contents,
//                     ));
//                     println!("Added new file to index: {}", path_str);
//                 }
//             }
//         }
//     }

//     // Commit the index
//     index_writer.commit()?;
//     println!("Time taken to add file to index: {:?}", start_time.elapsed());
//     Ok(())
// }

pub fn create_index_from_folder(index_path: &str, folder_path: &str) -> tantivy::Result<SystemTime> {
    // Start the timer
    let start_time = Instant::now();

    // Define the schema
    let mut schema_builder = Schema::builder();
    // Create the TextFieldIndexing object with positions
    let text_field_indexing = TextFieldIndexing::default()
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    // Create TextOptions with the indexing options
    let _text_field_options = TextOptions::default()
        .set_stored()
        .set_indexing_options(text_field_indexing);

    // schema_builder.add_text_field("path", text_field_options.clone());
    // schema_builder.add_text_field("contents", text_field_options.clone());
    schema_builder.add_text_field("path", STRING | STORED );
    schema_builder.add_text_field("contents", TEXT);

    let schema = schema_builder.build();

    // Create the index directory if it doesn't exist
    if Path::new(index_path).exists() {
        println!("Index already exists at {}", index_path);
        let metadata = metadata(index_path)?;
        let modified_time = metadata.modified()?;
        return Ok(modified_time);
    } else {
        fs::create_dir_all(index_path)?;
    }

    // Create an index in the specified directory
    let index = Index::create_in_dir(index_path, schema.clone())?;
    let mut index_writer = index.writer(15_000_000)?; // Increased buffer size

    // Recursively read files from the folder
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().display().to_string();
            if let Ok(contents) = fs::read_to_string(entry.path()) {
                // Optionally, limit the size of the contents
                let limited_contents = if contents.len() > 15_000_000 {
                    &contents[..15_000_000]
                } else {
                    &contents
                };
                // let limited_contents_with_path = String::from(format!("{}\n{}", path, limited_contents));
                // println!("Path added: {:?}", path);
                let _ = index_writer.add_document(doc!(
                    schema.get_field("path").unwrap() => path,
                    schema.get_field("contents").unwrap() => String::from(limited_contents),
                ));
            }
        }
    }

    // Commit the index
    index_writer.commit()?;

    // Measure the elapsed time
    let duration = start_time.elapsed();
    println!("Time taken to create index: {:?}", duration);
    // Get the last modified time of the index
    let metadata = metadata(index_path)?;
    let modified_time = metadata.modified()?;
    Ok(modified_time)
}

pub fn search_terms_in_index(index_path: &str, terms: &[&str]) -> tantivy::Result<Vec<(String, String)>> {
    let mut results = Vec::new();

    // Open the index
    let index = Index::open_in_dir(index_path)?;

    // Load the index schema
    let schema = index.schema();
    let path_field = schema.get_field("path").unwrap();
    let contents_field = schema.get_field("contents").unwrap();

    // Create a searcher
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;
    let searcher = reader.searcher();

    // Parse the query
    let query_parser = QueryParser::for_index(&index, vec![contents_field]);

    let start_time = Instant::now();
    for &term in terms {
        let query = query_parser.parse_query(term)?;
        // Search for the top 10000 documents
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10000))?;
        // Iterate over the search results and store them in the results vector
        for (_score, doc_address) in top_docs {
            let segment_reader: &SegmentReader = searcher.segment_reader(0);
            let is_deleted = segment_reader.is_deleted(doc_address.doc_id);

            if !is_deleted {
                let retrieved_doc : TantivyDocument = searcher.doc(doc_address)?;

                let path = retrieved_doc.get_first(path_field).and_then(|value| {
                    if let tantivy::schema::OwnedValue::Str(ref text) = value {
                        Some(text.as_str())
                    } else {
                        None
                    }
                }).unwrap_or("").to_string();

                let contents = retrieved_doc.get_first(contents_field).and_then(|value| {
                    if let tantivy::schema::OwnedValue::Str(ref text) = value {
                        println!("Not deleted {:?}", path.clone());
                        Some(text.as_str())
                    } else {
                        None
                    }
                }).unwrap_or("").to_string();

                results.push((path, contents));
            }
        }
    }
    // Measure the elapsed time
    let duration = start_time.elapsed();
    println!("Time taken to search term: {:?}", duration);


    Ok(results)
}

// pub fn search_phrases_in_index(index_path: &str, phrases: &[&str]) -> tantivy::Result<Vec<(String, String)>> {
//     let mut results = Vec::new();

//     // Open the index
//     let index = Index::open_in_dir(index_path)?;

//     // Load the index schema
//     let schema = index.schema();
//     let path_field = schema.get_field("path").unwrap();
//     let contents_field = schema.get_field("contents").unwrap();

//     // Create a searcher
//     let reader = index
//         .reader_builder()
//         .reload_policy(ReloadPolicy::Manual)
//         .try_into()?;
//     let searcher = reader.searcher();

//     let start_time = Instant::now();
//     // Parse the query
//     for &phrase in phrases {
//         println!("looking for {}", phrase);
//         let query_whitespace_split: Vec<&str> = phrase.split_whitespace().collect();
//         let terms: Vec<Term> = query_whitespace_split.iter().map(|term| Term::from_field_text(contents_field, term)).collect();
//         println!("looking for {:?}", terms);
//         let query = PhraseQuery::new(terms);
//         println!("looking for {:?}", query);
//         // Search for the top 10000 documents
//         let top_docs = searcher.search(&query, &TopDocs::with_limit(10000))?;
//         // Iterate over the search results and store them in the results vector
//         for (_score, doc_address) in top_docs {
//             println!("found term");
//             let retrieved_doc : TantivyDocument = searcher.doc(doc_address)?;

//             let path = retrieved_doc.get_first(path_field).and_then(|value| {
//                 if let tantivy::schema::OwnedValue::Str(ref text) = value {
//                     Some(text.as_str())
//                 } else {
//                     None
//                 }
//             }).unwrap_or("").to_string();

//             let contents = retrieved_doc.get_first(contents_field).and_then(|value| {
//                 if let tantivy::schema::OwnedValue::Str(ref text) = value {
//                     Some(text.as_str())
//                 } else {
//                     None
//                 }
//             }).unwrap_or("").to_string();

//             results.push((path, contents));
//         }
//     }
//     // Measure the elapsed time
//     let duration = start_time.elapsed();
//     println!("Time taken to search phrase: {:?}", duration);

//     Ok(results)
// }

pub fn search_phrases_in_index(index_path: &str, phrases: &[&str]) -> tantivy::Result<Vec<(String, String)>> {
    let mut results = Vec::new();

    // Open the index
    let index = Index::open_in_dir(index_path)?;

    // Load the index schema
    let schema = index.schema();
    let path_field = schema.get_field("path").unwrap();
    let contents_field = schema.get_field("contents").unwrap();

    // Create a searcher
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;
    let searcher = reader.searcher();

    // Parse the query
    let query_parser = QueryParser::for_index(&index, vec![contents_field]);

    let start_time = Instant::now();
    for &phrase in phrases {
        let query = query_parser.parse_query(&format!("\"{}\"", phrase))?;
        // Search for the top 10000 documents
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10000))?;
        // Iterate over the search results and store them in the results vector
        for (_score, doc_address) in top_docs {
            let segment_reader: &SegmentReader = searcher.segment_reader(0);
            let is_deleted = segment_reader.is_deleted(doc_address.doc_id);

            if !is_deleted {
                let retrieved_doc : TantivyDocument = searcher.doc(doc_address)?;
                let path = retrieved_doc.get_first(path_field).and_then(|value| {
                    if let tantivy::schema::OwnedValue::Str(ref text) = value {
                        Some(text.as_str())
                    } else {
                        None
                    }
                }).unwrap_or("").to_string();
                let contents = retrieved_doc.get_first(contents_field).and_then(|value| {
                    if let tantivy::schema::OwnedValue::Str(ref text) = value {
                        Some(text.as_str())
                    } else {
                        None
                    }
                }).unwrap_or("").to_string();

                results.push((path, contents));
            }
        }
    }
    println!("Time taken to search phrases: {:?}", start_time.elapsed());

    Ok(results)
}

pub fn search_regex_in_index(index_path: &str, regexes: &[&str]) -> tantivy::Result<Vec<(String, String)>> {
    let mut results = Vec::new();

    // Open the index
    let index = Index::open_in_dir(index_path)?;

    // Load the index schema
    let schema = index.schema();
    let path_field = schema.get_field("path").unwrap();
    let contents_field = schema.get_field("contents").unwrap();

    // Create a searcher
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()?;
    let searcher = reader.searcher();

    let start_time = Instant::now();

    for &regex_str in regexes {
        let regex = regex_str.to_string();
        let query = RegexQuery::from_pattern(&regex, contents_field)?;

        // println!("{:?}", query);
        // Search for the top 10000 documents
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10000))?;
        // Iterate over the search results and store them in the results vector
        for (_score, doc_address) in top_docs {
            let retrieved_doc : TantivyDocument = searcher.doc(doc_address)?;

            let path = retrieved_doc.get_first(path_field).and_then(|value| {
                if let tantivy::schema::OwnedValue::Str(ref text) = value {
                    Some(text.as_str())
                } else {
                    None
                }
            }).unwrap_or("").to_string();

            let contents = retrieved_doc.get_first(contents_field).and_then(|value| {
                if let tantivy::schema::OwnedValue::Str(ref text) = value {
                    Some(text.as_str())
                } else {
                    None
                }
            }).unwrap_or("").to_string();

            results.push((path, contents));
        }
    }
    println!("Time taken to search regex: {:?}", start_time.elapsed());

    Ok(results)
}

pub fn create_report(output_path: &str, results: &[(String, String)]) -> tantivy::Result<()> {
    let mut file = File::create(output_path)?;

    file.write_all(b"<!DOCTYPE html><html><head><title>Search Report</title></head><body>")?;
    file.write_all(b"<h1>Search Report</h1><ul>")?;

    for (path, _content) in results {
        file.write_all(format!("<li><b>File:</b> {:?}</li>", path).as_bytes())?;
    }

    file.write_all(b"</ul></body></html>")?;

    Ok(())
}


pub fn search(folder: &str, search_string: &str) {

    let string_type = check_string_type(search_string);
    let _home = "~/indexes/";
    let index_path = "ttv_idx_".to_owned() + folder.split('/').last().expect("REASON");
    println!("index path is {:?}", index_path);
    let search_terms: Vec<&str> = vec![search_string];
    let search_phrases: Vec<&str> = vec![search_string];
    let search_regex: Vec<&str> = vec![search_string];

    let folder_path = Path::new(folder).to_str();
    println!("folder path is {:?}", folder_path.unwrap());

    // Create index and get the index creation time
    let last_index_time = match create_index_from_folder(&index_path, folder_path.expect("REASON")) {
        Ok(modified_time) => {
            println!("Indexing completed successfully.");
            modified_time
        },
        Err(e) => {
            eprintln!("Failed to create index: {}", e);
            return;
        }
    };

    let _ = match update_index_with_new_files(&index_path, folder_path.expect("REASON"), last_index_time) {
        Ok(()) => {
            println!("Indexing completed successfully.");
        },
        Err(e) => {
            eprintln!("Failed to alter index: {}", e);
            return;
        }
    };

    println!("string_type = {:?}", string_type);
    if string_type == StringType::SingleTerm {
    // Perform searches and create reports
        match search_terms_in_index(&index_path, &search_terms) {
            Ok(results) => match create_report("search_terms_report.html", &results) {
                Ok(_) => println!("Search terms report created successfully."),
                Err(e) => eprintln!("Failed to create search terms report: {}", e),
            },
            Err(e) => eprintln!("Failed to search terms: {}", e),
        }

    } else if string_type == StringType::Phrase {
        match search_phrases_in_index(&index_path, &search_phrases) {
            Ok(results) => match create_report("search_phrases_report.html", &results) {
                Ok(_) => println!("Search phrases report created successfully."),
                Err(e) => eprintln!("Failed to create search phrases report: {}", e),
            },
            Err(e) => eprintln!("Failed to search phrases: {}", e),
        }
    } else if string_type == StringType::Regex {
        match search_regex_in_index(&index_path, &search_regex) {
            Ok(results) => match create_report("search_regex_report.html", &results) {
                Ok(_) => println!("Search regex report created successfully."),
                Err(e) => eprintln!("Failed to create search regex report: {}", e),
            },
            Err(e) => eprintln!("Failed to search regex: {}", e),
        }
    }
}
