use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    body: Body,
    query_string_parameters: QueryStringParameters,
    path_parameters: PathParameters,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Body {
    foo: String,
    bar: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct QueryStringParameters {
    baz: Baz,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Baz {
    Baz1,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PathParameters {
    id: FooId,
}

#[derive(Serialize, Deserialize)]
pub struct FooId(Ulid);

impl JsonSchema for FooId {
    fn schema_name() -> String {
        "FooId".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("ulid".to_string()),
            ..Default::default()
        }
        .into()
    }
}
