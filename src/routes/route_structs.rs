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

///  A request to get similar strings by embedding
/// * `search_type` : type of searching algorithm you want to perform on the vectors.    
/// * `string_embedding`: string as a vector representation (user is responsible for this)
/// * `inlcude_embedding`: by default , db_api doesn't include embeddings in the response for
/// better performance as embeddings are very large lists
/// * `k` : the number of elements to include in the results 
/// * `userid`: the merchant's uuid in the database
///    
///
/// 
/// *this is a sample request*
///
///```
/// {
///     "string_embedding":[*your string embedding vector*],
///        "k" : 20,
///        "userid": "84f4cdaa-6c96-4e94-95a2-dbd88ef18446",
///        "search_type": {"MmrSimilarity" :
///                                         {"similarity": 0.85,
///                                          "diversity": 0.1}} ,
///        "include_embedding": false
/// }
///```
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
