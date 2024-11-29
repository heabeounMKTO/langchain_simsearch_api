mod route_structs;

use crate::operators::query::{
    generate_similarity_search_query, SimilaritySearchType, SimilarityThreshold,
};
use crate::utils::StringEmbeddingPair;
use actix_web::http::header::ContentType;
use actix_web::{get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer};
use deadpool_postgres::{GenericClient, Pool};
use pgvector::Vector;
use route_structs::{AllIndexesResponse, SimilarStringRequest, SimilarStringResponse};
use uuid::{uuid, Uuid};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("hehe")
}

#[get("/all_indexes")]
pub async fn get_all_indexes(pool: web::Data<Pool>, _: HttpRequest) -> HttpResponse {
    let client = pool.get().await.unwrap();
    let mut all_indexes = vec![];
    let rows = client
        .query("SELECT * FROM langchain_pg_collection;", &[])
        .await
        .unwrap();
    if rows.len() > 0 {
        let _a: Vec<String> = rows
            .iter()
            .filter_map(|row| row.get::<_, Option<String>>("name"))
            .collect();
        all_indexes = _a
    };
    HttpResponse::Ok().json(AllIndexesResponse {
        index_names: all_indexes,
        status: 200,
        message: String::from("success"),
    })
}

/// by defaults omits the embedding unless asked,
/// for better bandwidth and response speed!
#[post("/similar_strings")]
pub async fn get_similar_strings(
    pool: web::Data<Pool>,
    form: web::Json<SimilarStringRequest>,
    _: HttpRequest,
) -> HttpResponse {
    // parse da json optiionZ
    let _k: i64 = match form.k {
        Some(k_val) => k_val as i64,
        None => 4,
    };

    let include_embedding: bool = match form.include_embedding {
        Some(inc) => inc,
        None => false,
    };
    let placeholder_collection_name = form.userid.parse::<Uuid>().unwrap();

    let _search_type: SimilaritySearchType = match &form.search_type {
        Some(ref _a) => _a.clone(),
        None => SimilaritySearchType::CosineSimilarity(SimilarityThreshold { similarity: 0.8 }),
    };
    let client = pool.get().await.unwrap();
    let _string_embedding = pgvector::Vector::from(form.string_embedding.to_owned());
    let rows = match _search_type {
        SimilaritySearchType::MmrSimilarity(_mmr) => {
            let _diversity: i32 = _mmr.diversity as i32;
            let _query = generate_similarity_search_query(&_search_type).unwrap();
            client
                .query(
                    _query,
                    &[
                        &_string_embedding,
                        &placeholder_collection_name,
                        &(_k as i32),
                        &_diversity,
                        &_mmr.similarity,
                    ],
                )
                .await
                .unwrap()
        }
        _ => {
            let _threshold = match &_search_type {
                SimilaritySearchType::CosineSimilarity(_a) => _a.similarity,
                SimilaritySearchType::L1Similarity(_a) => _a.similarity,
                SimilaritySearchType::L2Similarity(_a) => _a.similarity,
                _ => 0.5,
            };
            let _query = generate_similarity_search_query(&_search_type).unwrap();
            client
                .query(
                    _query,
                    &[
                        &_string_embedding,
                        &placeholder_collection_name,
                        &_k,
                        &_threshold,
                    ],
                )
                .await
                .unwrap()
        }
    };
    let similar_str = {
        if rows.len() > 0 {
            let res: Vec<StringEmbeddingPair> = rows
                .into_iter()
                .map(|row| {
                    if include_embedding {
                        StringEmbeddingPair {
                            embedding: Some(row.get::<_, pgvector::Vector>("embedding").to_vec()),
                            raw_string: row.get("document"),
                            similarity: row.get("similarity"),
                        }
                    } else {
                        StringEmbeddingPair {
                            embedding: None,
                            raw_string: row.get("document"),
                            similarity: row.get("similarity"),
                        }
                    }
                })
                .collect();
            res
        } else {
            let _empty: Vec<StringEmbeddingPair> = vec![];
            _empty
        }
    };

    HttpResponse::Ok().json(SimilarStringResponse {
        message: String::from("success"),
        search_type: _search_type,
        status: 200,
        similar_strings: similar_str,
    })
}
