pub struct Message {
    pub message: String,
}

#[async_graphql::Object]
impl Message {
    async fn message(&self) -> &str {
        &self.message
    }
}