use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use serde::{Deserialize};


#[derive(Deserialize)]
struct AnswerRequest {
    question: String,
    context: String
}

#[post("/")]
async fn answer_question(answer_request: web::Json<AnswerRequest>) -> HttpResponse {
    let question = answer_request.question.to_string();
    let print_question = question.clone();

    let context = answer_request.context.to_string();
    let print_context = context.clone();

    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();

    let answer = qa_model.predict(&[QaInput {question, context}], 1, 32);

    let response_body = format!(
        "Question: {:#?},\n Context: {:#?},\n Answer {:#?},\n",
        print_question,
        print_context,
        answer
    );

    HttpResponse::Ok().body(response_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(answer_question)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
