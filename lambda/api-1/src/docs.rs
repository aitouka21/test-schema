mod request;
mod response;

use std::fs;

use request::Request;
use response::Response;

use schemars::schema_for;
use serde_json::to_string_pretty;

fn main() -> std::io::Result<()> {
    let request_schema = schema_for!(Request);

    fs::write(
        "schemas/Api1Request.json",
        to_string_pretty(&request_schema).unwrap(),
    )?;

    let response_schema = schema_for!(Response);
    fs::write(
        "schemas/Api1Response.json",
        to_string_pretty(&response_schema).unwrap(),
    )?;

    Ok(())
}
