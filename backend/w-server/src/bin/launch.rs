use actix_web::web::head;
use actix_web::{web, web::Data, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;
use w_db::establish_connection;
use w_external_api::GptClient;
use w_server::{ContextData, Query};

async fn api_handler(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
    data: web::Data<ContextData>,
) -> GraphQLResponse {
    let request = &req.0;
    let query = &request.query;
    let vars = &request.variables;
    println!("query : {:#?}", query);
    println!("vars : {:#?}", vars);
    let mut req = req.into_inner();
    req = req.data(data);

    schema.execute(req).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:5000");
    let pool = establish_connection().await.unwrap();

    let gpt_client = GptClient::new();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Schema::new(
                Query,
                EmptyMutation,
                EmptySubscription,
            )))
            .app_data(web::Data::new(ContextData {
                pool: pool.clone(),
                gpt_client: gpt_client.clone(),
            }))
            .service(web::resource("/").to(api_handler))
            .service(
                web::scope("/api")
                    .service(web::resource("/graphql").route(web::post().to(api_handler))),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
