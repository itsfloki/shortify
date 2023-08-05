use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Links {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub url: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReqBody {
    pub url: String,
}
