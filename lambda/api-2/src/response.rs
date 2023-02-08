use lambda_http::aws_lambda_events::chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct Response {
    foo: String,
    bar: String,
    date: DateTime<Utc>,
}
