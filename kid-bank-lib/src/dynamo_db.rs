#[derive(Debug)]
struct DynamoClient {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl DynamoClient {
    pub fn new(config: &aws_sdk_dynamodb::Config) -> Result<Self, String> {
        let client = aws_sdk_dynamodb::Client::new(config);

        Ok(DynamoClient {
            table_name: config.app_name,
            client,
        })
    }

    pub fn query(&self) {}
}
