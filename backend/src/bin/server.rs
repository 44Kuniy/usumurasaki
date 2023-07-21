use actix_web::{web, web::Data, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use usumurasaki::graphql::query::Query;

async fn index(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let request = &req.0;
    let query = &request.query;
    let vars = &request.variables;
    println!("query : {:#?}", query);
    println!("vars : {:#?}", vars);

    schema.execute(req.into_inner()).await.into()
}

// async fn gql_playgound() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(GraphiQLSource::build().endpoint("/").finish())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:5000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Schema::new(
                Query,
                EmptyMutation,
                EmptySubscription,
            )))
            .service(web::resource("/").to(index))
            .service(
                web::scope("/api").service(web::resource("/graphql").route(web::post().to(index))),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
