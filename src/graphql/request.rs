use std::io::Read;
use juniper::Variables;
use serde_json::{from_str, Value};
use rocket::{Request as RocketRequest, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::Outcome::*;

#[derive(Debug)]
pub struct Request {
    pub query: String,
    pub variables: Variables,
    pub operation_name: Option<String>
}

impl FromData for Request {
    type Error = String;

    fn from_data(
        req: &RocketRequest, data: Data
    ) -> data::Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        let content_type = ContentType::new("application", "json");
        if req.content_type() != Some(content_type) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        // Parse JSON body
        let json: Value = match from_str(&string[..]) {
            Ok(value) => value,
            Err(_) => return Failure(
                (Status::BadRequest, "Body isn't a valid JSON".to_string())
            )
        };

        // Extract the query string
        let query: String = match json["query"].as_str() {
            Some(value) => value.to_string(),
            _ => return Failure(
                (
                    Status::BadRequest,
                    "Body is missing value for `query`".to_string()
                )
            )
        };

        // Extract operation name
        let operation_name = match json["operationName"].as_str() {
            Some(value) => Some(value.to_string()),
            _ => None
        };

        let variables = Variables::new();

        // if json["variables"] != json!(null) {
        //     variables = InputValue::from_json(json["variables"]).to_object_value()
        //         .map(|o| o.into_iter().map(|(k, v)| (k.to_owned(), v.clone())).collect())
        //         .unwrap_or_default();
        // }

        // Return successfully.
        Success(Request {
            query: query,
            variables: variables,
            operation_name: operation_name
        })
    }
}

