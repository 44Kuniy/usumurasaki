use actix_web::{web, web::Data, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;
use usumurasaki::database::establish_connection;
use usumurasaki::graphql::query::Query;

struct ContextData {
    pub pool: PgPool,
}

async fn index(
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
    let pool = data.pool.clone();
    req = req.data(pool);

    schema.execute(req).await.into()
}

// async fn gql_playgound() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(GraphiQLSource::build().endpoint("/").finish())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:5000");
    let pool = establish_connection().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Schema::new(
                Query,
                EmptyMutation,
                EmptySubscription,
            )))
            .app_data(web::Data::new(ContextData { pool: pool.clone() }))
            .service(web::resource("/").to(index))
            .service(
                web::scope("/api").service(web::resource("/graphql").route(web::post().to(index))),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
