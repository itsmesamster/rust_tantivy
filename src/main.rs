use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: folder_indexer <folder_path>");
        return;
    }

    let folder_path = &args[1];
    let index_path = "wiki_1_index";
    let search_terms: Vec<&str> = vec!["Australia"];
    let search_phrases: Vec<&str> = vec!["Cross Roads, Ripley County, Indiana"];
    let search_regex: Vec<&str> = vec!["d[ai]{2}ry"];

    // match rust_tantivy::create_index_from_folder(index_path, folder_path) {
    //     Ok(_) => {
    //         println!("Indexing completed successfully.");
    //         let _ = rust_tantivy::watch_folder_for_changes(&index_path, &folder_path).unwrap();

    //         // Search using terms
    //         match rust_tantivy::search_terms_in_index(index_path, &search_terms) {
    //             Ok(results) => match rust_tantivy::create_report("search_terms_report.html", &results) {
    //                 Ok(_) => println!("Search terms report created successfully."),
    //                 Err(e) => eprintln!("Failed to create search terms report: {}", e),
    //             },
    //             Err(e) => eprintln!("Failed to search terms: {}", e),
    //         }

    //         // Search using phrases
    //         match rust_tantivy::search_phrases_in_index(index_path, &search_phrases) {
    //             Ok(results) => match rust_tantivy::create_report("search_phrases_report.html", &results) {
    //                 Ok(_) => println!("Search phrases report created successfully."),
    //                 Err(e) => eprintln!("Failed to create search phrases report: {}", e),
    //             },
    //             Err(e) => eprintln!("Failed to search phrases: {}", e),
    //         }

    //         // Search using regex
    //         match rust_tantivy::search_regex_in_index(index_path, &search_regex) {
    //             Ok(results) => match rust_tantivy::create_report("search_regex_report.html", &results) {
    //                 Ok(_) => println!("Search regex report created successfully."),
    //                 Err(e) => eprintln!("Failed to create search regex report: {}", e),
    //             },
    //             Err(e) => eprintln!("Failed to search regex: {}", e),
    //         }
    //     }
    //     Err(e) => eprintln!("Failed to create index: {}", e),
    // }
    // Create index and get the index creation time
    let last_index_time = match rust_tantivy::create_index_from_folder(index_path, folder_path) {
        Ok(modified_time) => {
            println!("Indexing completed successfully.");
            modified_time
        },
        Err(e) => {
            eprintln!("Failed to create index: {}", e);
            return;
        }
    };

    let _ = match rust_tantivy::update_index_with_new_files(index_path, folder_path, last_index_time) {
        Ok(()) => {
            println!("Indexing completed successfully.");
        },
        Err(e) => {
            eprintln!("Failed to alter index: {}", e);
            return;
        }
    };

    // Perform searches and create reports
    match rust_tantivy::search_terms_in_index(index_path, &search_terms) {
        Ok(results) => match rust_tantivy::create_report("search_terms_report.html", &results) {
            Ok(_) => println!("Search terms report created successfully."),
            Err(e) => eprintln!("Failed to create search terms report: {}", e),
        },
        Err(e) => eprintln!("Failed to search terms: {}", e),
    }

    match rust_tantivy::search_phrases_in_index(index_path, &search_phrases) {
        Ok(results) => match rust_tantivy::create_report("search_phrases_report.html", &results) {
            Ok(_) => println!("Search phrases report created successfully."),
            Err(e) => eprintln!("Failed to create search phrases report: {}", e),
        },
        Err(e) => eprintln!("Failed to search phrases: {}", e),
    }

    match rust_tantivy::search_regex_in_index(index_path, &search_regex) {
        Ok(results) => match rust_tantivy::create_report("search_regex_report.html", &results) {
            Ok(_) => println!("Search regex report created successfully."),
            Err(e) => eprintln!("Failed to create search regex report: {}", e),
        },
        Err(e) => eprintln!("Failed to search regex: {}", e),
    }
}
