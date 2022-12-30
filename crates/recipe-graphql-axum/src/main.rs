use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use recipe_db::create_pool;
use recipe_graphql::{create_schema, Schema};

async fn graphql_handler(schema: State<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = create_schema(create_pool().expect("could not connect to database"));

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .with_state(schema);

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
