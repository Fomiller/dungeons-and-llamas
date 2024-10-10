// use aws_config::BehaviorVersion;
//
// pub struct AWS {
//     config: aws_config::SdkConfig,
//     ddb_client: Option<aws_sdk_dynamodb::Client>,
// }
//
// impl AWS {
//     async pub fn default()
//     pub fn new_ddb_client() -> Self {
//         let config = aws_config::defaults(BehaviorVersion::latest())
//             .region("us-east-1")
//             .load()
//             .await;
//         let client = aws_sdk_dynamodb::Client::new(&config);
//
//         Self { client }
//     }
// }
