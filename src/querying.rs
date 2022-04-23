mod schema {
    cynic::use_schema!("schemas\\meetup.schema.graphql");
}

use cynic::QueryBuilder;
use cynic::http::SurfExt;
use cynic::Operation;

// Generated using cynic's query generator
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schemas\\meetup.schema.graphql",
    query_module = "schema",
)]
pub struct User {
    pub id: cynic::Id,
    pub name: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schemas\\meetup.schema.graphql",
    graphql_type = "Query"
)]
pub struct MyQuery {
    #[cynic(rename = "self")]
    pub self_: Option<User>,
}


/**
 * Makes the query
 */
pub async fn perform_query(token: String) /*-> GraphQlResponse<MyQuery>*/ {
    let operation = build_query();
    println!("{:?}", surf::post("https://swapi-graphql.netlify.app/.netlify/functions/index")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .run_graphql(operation)
        .await
        .unwrap());
}

pub fn build_query() -> Operation<'static, MyQuery> {
    MyQuery::build(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot_test_query() {
        let query = build_query();

        insta::assert_snapshot!(query.query);
    }
}