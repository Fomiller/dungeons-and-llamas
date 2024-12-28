use anyhow;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

pub mod queries {
    include!(concat!("./queries", "/classes_query.rs"));
}

use queries::get_classes::Variables;

pub async fn get_classes() -> anyhow::Result<()> {
    let endpoint = "https://www.dnd5eapi.co/graphql";

    let client = Client::new();

    let variables = Variables {};

    let request_body = queries::GetClasses::build_query(variables);

    let response = client
        .post(endpoint)
        .json(&request_body)
        .send()
        .await?
        .json::<Response<queries::get_classes::ResponseData>>()
        .await?;

    if let Some(data) = response.data {
        for class in data.classes {
            println!(
                "Index: {:?}, Name: {:?}",
                class.index.unwrap(),
                class.name.unwrap()
            );
        }
    } else {
        eprintln!("GraphQL Errors: {:?}", response.errors);
    }

    Ok(())
}

