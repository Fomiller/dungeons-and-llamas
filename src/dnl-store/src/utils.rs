use aws_sdk_dynamodb::operation::query::QueryOutput;
use dnl_types::error::StoreError as Error;
use lambda_http::tracing::info;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait QueryOutputToVec {
    fn to_vec<T: Serialize + DeserializeOwned>(self, key: &str) -> Result<Option<Vec<T>>, Error>;
}

impl QueryOutputToVec for QueryOutput {
    fn to_vec<T: Serialize + serde::de::DeserializeOwned>(
        self,
        key: &str,
    ) -> Result<Option<Vec<T>>, Error> {
        let items: Vec<T> = Vec::new();

        if let Some(_items) = self.items {
            if _items.iter().all(|map| map.is_empty()) {
                Ok(Some(items))
            } else {
                info!("Items: {:?}", _items);

                let item = _items
                    .first()
                    .expect(&format!("{} should have a length of at least one", key));

                let value = item
                    .get("items")
                    .expect(&format!("{} should be a valid key", key));

                let _items: Vec<_> = value
                    .as_l()
                    .expect(&format!("{} value should be a list", key))
                    .to_owned()
                    .into_iter()
                    .filter_map(|v| v.as_m().ok().cloned())
                    .collect();

                let items: Vec<T> = serde_dynamo::from_items(_items)
                    .map_err(|e| Error::SerdeDynamo(e.to_string()))?;

                Ok(Some(items))
            }
        } else {
            Ok(None)
        }
    }
}
