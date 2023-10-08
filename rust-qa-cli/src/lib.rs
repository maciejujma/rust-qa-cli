// Add required imports
extern crate wikipedia;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use wikipedia::http::default::Client;
use wikipedia::Wikipedia;

// Function to get the content from a Wikipedia page
pub fn get_wiki_content(page: &str) -> String {
    // Change to the correct type parameter for Wikipedia
    let wiki = Wikipedia::<Client>::default();
    let page = wiki.page_from_title(page.to_owned());

    // Check if the page exists before trying to get the content
    match page.get_content() {
        Ok(content) => content,
        Err(_) => String::from("Error: Could not retrieve the content."),
    }
}

//Function to answer a question
pub fn answer_question(question: &str, context: &str) -> Vec<String> {
    //load a model
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let question = QaInput {
        question: question.to_string(),
        context: context.to_string(),
    };
    //return output as a vector of strings
    let output = qa_model.predict(&[question], 1, 32);
    output[0].iter().map(|x| x.answer.to_string()).collect()
}
