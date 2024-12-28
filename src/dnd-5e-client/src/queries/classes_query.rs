#[allow(clippy::all, warnings)]
pub struct GetClasses;
pub mod get_classes {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "GetClasses";
    pub const QUERY: &str = "query GetClasses {\n  classes {\n    index\n    name\n  }\n}\n";
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub classes: Vec<GetClassesClasses>,
    }
    #[derive(Deserialize)]
    pub struct GetClassesClasses {
        pub index: Option<String>,
        pub name: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for GetClasses {
    type Variables = get_classes::Variables;
    type ResponseData = get_classes::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_classes::QUERY,
            operation_name: get_classes::OPERATION_NAME,
        }
    }
}
