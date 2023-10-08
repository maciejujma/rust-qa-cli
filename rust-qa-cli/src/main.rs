use clap::{command, Arg};

fn main() {
    // Creating CLI
    let match_result: clap::ArgMatches = command!()
        .about("This application answers question based on the context.")
        .arg(
            Arg::new("question")
                .short('q')
                .long("question")
                .required(true)
                .help("Question to be answered."),
        )
        .arg(
            Arg::new("context")
                .short('c')
                .long("context")
                .required(true)
                .conflicts_with("wikipedia")
                .help("Context required to answer the question."),
        )
        .arg(
            Arg::new("wikipedia")
                .short('w')
                .long("wikipedia")
                .required(true)
                .conflicts_with("context")
                .help("Title of the wikipedia's page required to answer the question."),
        )
        .get_matches();

    // Getting question from the user's input
    let question: &String = match_result.get_one::<String>("question").unwrap();

    // Depeding on the user's choice
    if match_result.contains_id("context") {
        // Getting provided context
        let context: &String = match_result.get_one::<String>("context").unwrap();

        // Using context to answer the question
        let answer = rust_qa_cli::answer_question(question, context);

        // Printing the question and the answer
        println!("Question: {}", question);
        println!("Answer: {}", answer[0]);
    } else if match_result.contains_id("wikipedia") {
        // Getting provided Wikipedia's page title
        let wikipedia: &String = match_result.get_one::<String>("wikipedia").unwrap();

        // Loading Wikipedia's page
        let context: String = rust_qa_cli::get_wiki_content(wikipedia);

        // Using Wikipedia's page as a context to answer the question
        let answer = rust_qa_cli::answer_question(question, &context);

        // Printing the question and the answer
        println!("Question: {}", question);
        println!("Answer: {}", answer[0]);
    }
}
