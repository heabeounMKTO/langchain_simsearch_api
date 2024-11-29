use actix_multipart::form::MultipartForm;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::utils::Interval;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SimilaritySearchType {
    MmrSimilarity(MmrSimilarityThreshold),
    L1Similarity(SimilarityThreshold),
    L2Similarity(SimilarityThreshold),
    CosineSimilarity(SimilarityThreshold),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SimilarityThreshold {
    pub similarity: f64,
}
/// TODO: just make it `SimilarityThreshold` holy shit
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct MmrSimilarityThreshold {
    pub similarity: f64,
    pub diversity: f64,
}

impl Interval for SimilarityThreshold {
    fn check_bounds(&self, min: f64, max: f64) -> bool {
        if self.similarity >= min && self.similarity <= max {
            true
        } else {
            false
        }
    }
}

impl Interval for MmrSimilarityThreshold {
    fn check_bounds(&self, min: f64, max: f64) -> bool {
        if self.similarity >= min && self.similarity <= max {
            if self.diversity >= min && self.diversity <= max {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// generates a `pgvector` compatible SQL query based on the search type
/// provided.
///
/// reason we using $ instead of just inserting numbers is for consistency
/// (copy pasting SQL queries)
/// note: $1 is meant for the vector you want to query
///       $2 is the collection_id or merchant's index
///       $3 is the limit to search
///       $4 (optional) is the diversity for MMR similarity
pub fn generate_similarity_search_query(
    search_type: &SimilaritySearchType,
) -> Result<&'static str, Error> {
    let search_string: Result<&str, Error> = match search_type {
        SimilaritySearchType::CosineSimilarity(_) => Ok(
            "SELECT document, cmetadata,embedding, 1 - (embedding <=> $1) 
                                                    AS similarity
                                                    FROM langchain_pg_embedding 
                                                    WHERE collection_id=$2 
                                                        AND (1 - (embedding <=> $1)) >= $4
                                                    ORDER BY similarity DESC LIMIT $3",
        ),
        SimilaritySearchType::L1Similarity(_) => {
            Ok("SELECT document, cmetadata,embedding, embedding <+> $1  
                                                    AS similarity
                                                    FROM langchain_pg_embedding 
                                                    WHERE collection_id=$2 
                                                        AND (1 - (embedding <=> $1)) >= $4
                                                    ORDER BY similarity DESC LIMIT $3")
        }
        SimilaritySearchType::L2Similarity(_) => {
            Ok("SELECT document, cmetadata,embedding, embedding <-> $1  
                                                    AS similarity
                                                    FROM langchain_pg_embedding 
                                                    WHERE collection_id=$2 
                                                        AND (1 - (embedding <=> $1)) >= $4
                                                    ORDER BY similarity DESC LIMIT $3")
        }
        SimilaritySearchType::MmrSimilarity(_) => Ok("
                  WITH ranked_candidates AS (
                    SELECT 
                        document, 
                        cmetadata, 
                        embedding, 
                        1 - (embedding <=> $1) AS similarity
                    FROM langchain_pg_embedding 
                    WHERE collection_id = $2 
                        AND (1 - (embedding <=> $1)) >= $5
                    ORDER BY similarity DESC
                    LIMIT $3 * 2
                ),
                first_result AS (
                    SELECT 
                        document, 
                        cmetadata, 
                        embedding, 
                        similarity
                    FROM ranked_candidates
                    ORDER BY similarity DESC
                    LIMIT 1
                ),
                mmr_results AS (
                    SELECT 
                        rc.document, 
                        rc.cmetadata, 
                        rc.embedding, 
                        rc.similarity,
                        (1 - $4) * rc.similarity + 
                        $4 * (rc.embedding <=> fr.embedding) AS mmr_score
                    FROM ranked_candidates rc
                    CROSS JOIN first_result fr
                    WHERE rc.document != fr.document
                    ORDER BY mmr_score DESC
                    LIMIT $3 - 1
                )
                SELECT 
                    document, 
                    cmetadata, 
                    similarity
                FROM (
                    SELECT 
                        document, 
                        cmetadata, 
                        similarity,
                        1 AS priority
                    FROM first_result

                    UNION ALL

                    SELECT 
                        document, 
                        cmetadata, 
                        similarity,
                        2 AS priority
                    FROM mmr_results
                ) combined
                ORDER BY priority, similarity DESC
                LIMIT $3;
            "),
    };
    search_string
}
