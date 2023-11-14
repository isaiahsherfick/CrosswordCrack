use clap::{App, Arg};

fn main() {
    let matches = App::new("Crossword Crack")
        .version("1.0")
        .author("Your Name")
        .about("A crossword puzzle solving tool")
        .arg(Arg::with_name("pattern")
            .short('p')
            .long("pattern")
            .takes_value(true)
            .help("Sets the pattern for the word to search for. To find all 7-letter words with an R in the middle, for example: ***R*** ")
        )
        .arg(Arg::with_name("synonyms")
            .short('s')
            .long("synonyms")
            .takes_value(true)
            .use_delimiter(true)
            .multiple(true)
            .help("Sets synonyms for the word. Example usage: crosswordcrack --synonyms [bad, poor, lackluster]")
        )
        .arg(Arg::with_name("definition_contains")
            .short('d')
            .long("definition-contains")
            .takes_value(true)
            .use_delimiter(true)
            .multiple(true)
            .help("Search for keywords in the definition using regular expressions. Example usage: crosswordcrack --definition_contains regex")
        )
        .get_matches();

    let synonyms_list: Vec<&str>;
    let keywords_list: Vec<&str>;
    let pattern: &str;


    if let Some(pattern) = matches.value_of("pattern") {
        // Process pattern argument
        println!("Pattern: {}", pattern);
    }

    if let Some(synonyms) = matches.values_of("synonyms") {
        // Process synonyms argument
        synonyms_list = synonyms.collect();
        println!("Synonyms: {:?}", synonyms_list);
    }

    if let Some(definition_contains) = matches.values_of("definition_contains") {
        // Process definition_contains argument
        let keywords_list: Vec<&str> = definition_contains.collect();
        println!("Definition contains: {:?}", keywords_list);
    }
    handle_args(pattern,synonyms_list,keywords_list);
}


fn handle_args(pattern: &str,synonyms: Vec<&str>,definition_contains: Vec<&str>) {
    if (pattern.is_null() && synonyms.is_null() && definition_contains.is_null()) {
        println!("Usage: crosswordcrack --pattern --definition_contains --synonyms\nSee help options for individual params for more information.");
    }
}
