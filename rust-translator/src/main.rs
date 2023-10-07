use clap::{command, Arg};

fn main() {
    let match_result: clap::ArgMatches = command!()
        .about("This application translates sentence.")
        .arg(
            Arg::new("french")
                .short('f')
                .long("french")
                .aliases(["fr", "french-sentence"])
                .required(true)
                .conflicts_with("english")
                .help("The sentence in french."),
        )
        .arg(
            Arg::new("english")
                .short('e')
                .long("english")
                .aliases(["eng", "english-sentence"])
                .required(true)
                .conflicts_with("french")
                .help("The sentence in english."),
        )
        .get_matches();

    if match_result.contains_id("french") {
        let french_sentence: &String = match_result.get_one::<String>("french").unwrap();
        println!("Sentence in french: {}", french_sentence);
    } else if match_result.contains_id("english") {
        let english_sentence: &String = match_result.get_one::<String>("english").unwrap();
        println!("Sentence in english: {}", english_sentence);
    }
}
