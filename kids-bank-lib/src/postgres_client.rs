pub struct PSQLClient {
    table_name: String,
    port: i32,
    client: aws_sdk_rds::Client,
}

impl PSQLClient {
    pub fn new(
        config: &aws_config::SdkConfig,
        table_name: &str,
        port: i32,
    ) -> Result<Self, String> {
        let client = aws_sdk_rds::Client::new(config);
        Ok(PSQLClient {
            table_name: table_name.to_string(),
            port,
            client,
        })
    }
}
