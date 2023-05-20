use actix_web::{
    App, guard, HttpResponse, HttpServer, web,
};
use async_graphql::{http::GraphiQLSource, Schema, EmptySubscription};
use async_graphql_actix_web::{GraphQLResponse, GraphQLRequest};
use crate::resolvers::graphql_schema::{SchemaQL, QueryRoot, MutationRoot};

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .finish(),
        )
}

async fn index(
    schema: web::Data<SchemaQL>,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_request.into_inner();
    schema.execute(request).await.into()
}

pub async fn run_server() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}