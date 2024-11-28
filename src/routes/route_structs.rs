//// structs used in all of the routes!
use crate::operators::query::SimilaritySearchType;
///
use crate::utils::StringEmbeddingPair;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AllIndexesResponse {
    pub index_names: Vec<String>,
    pub message: String,
    pub status: u32,
}

/// get similar strings by embedding
#[derive(Deserialize, Serialize, Debug)]
pub struct SimilarStringRequest {
    pub search_type: Option<SimilaritySearchType>,
    pub string_embedding: Vec<f32>,
    pub include_embedding: Option<bool>,
    pub k: Option<u32>,
    pub userid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SimilarStringResponse {
    pub similar_strings: Vec<StringEmbeddingPair>,
    pub message: String,
    pub status: u32,
    pub search_type: SimilaritySearchType,
}
