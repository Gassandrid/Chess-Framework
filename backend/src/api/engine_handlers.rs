use crate::engine::{
    perft::Perft,
    position::Position as EnginePosition,
    search::{Search, SearchLimits},
};
use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct EngineMoveRequest {
    pub fen: String,
    pub depth: Option<u8>,
    pub time_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct EngineMoveResponse {
    pub best_move: String,
    pub score: Option<i32>,
    pub nodes: u64,
    pub time_ms: u128,
    pub nps: u64,
}

#[derive(Debug, Deserialize)]
pub struct EngineAnalysisRequest {
    pub fen: String,
    pub depth: u8,
}

#[derive(Debug, Serialize)]
pub struct EngineAnalysisResponse {
    pub best_move: String,
    pub evaluation: i32,
    pub nodes: u64,
    pub nps: u64,
}

#[derive(Debug, Deserialize)]
pub struct PerftRequest {
    pub fen: Option<String>,
    pub depth: u8,
}

#[derive(Debug, Serialize)]
pub struct PerftResponse {
    pub nodes: u64,
    pub time_ms: u128,
}

/// Get the best move from the engine
pub async fn get_best_move(
    Json(request): Json<EngineMoveRequest>,
) -> Result<Json<EngineMoveResponse>, StatusCode> {
    // Parse FEN
    let position = EnginePosition::from_fen(&request.fen)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Set up search
    let mut search = Search::new(128); // 128 MB transposition table
    let mut limits = SearchLimits::default();

    if let Some(depth) = request.depth {
        limits.max_depth = Some(depth);
    }

    if let Some(time) = request.time_ms {
        limits.max_time = Some(Duration::from_millis(time));
    }

    // Search for best move
    let best_move = search.search(&position, limits);

    if let Some(mv) = best_move {
        let stats = search.get_stats();

        Ok(Json(EngineMoveResponse {
            best_move: mv.to_uci(),
            score: None,
            nodes: stats.nodes + stats.qnodes,
            time_ms: stats.time_elapsed.as_millis(),
            nps: stats.nps(),
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Analyze a position
pub async fn analyze_position(
    Json(request): Json<EngineAnalysisRequest>,
) -> Result<Json<EngineAnalysisResponse>, StatusCode> {
    // Parse FEN
    let position = EnginePosition::from_fen(&request.fen)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get static evaluation
    let evaluation = crate::engine::evaluation::Evaluator::evaluate(&position);

    // Set up search
    let mut search = Search::new(128);
    let limits = SearchLimits {
        max_depth: Some(request.depth),
        ..Default::default()
    };

    // Search for best move
    let best_move = search.search(&position, limits);

    if let Some(mv) = best_move {
        let stats = search.get_stats();

        Ok(Json(EngineAnalysisResponse {
            best_move: mv.to_uci(),
            evaluation,
            nodes: stats.nodes + stats.qnodes,
            nps: stats.nps(),
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Run perft test
pub async fn perft_test(
    Json(request): Json<PerftRequest>,
) -> Result<Json<PerftResponse>, StatusCode> {
    // Parse FEN or use starting position
    let position = if let Some(fen) = request.fen {
        EnginePosition::from_fen(&fen).map_err(|_| StatusCode::BAD_REQUEST)?
    } else {
        EnginePosition::startpos()
    };

    // Run perft
    let start = std::time::Instant::now();
    let nodes = Perft::perft(&position, request.depth);
    let elapsed = start.elapsed();

    Ok(Json(PerftResponse {
        nodes,
        time_ms: elapsed.as_millis(),
    }))
}

/// Get static evaluation of a position
#[derive(Debug, Deserialize)]
pub struct EvaluationRequest {
    pub fen: String,
}

#[derive(Debug, Serialize)]
pub struct EvaluationResponse {
    pub evaluation: i32,
    pub description: String,
}

pub async fn evaluate_position(
    Json(request): Json<EvaluationRequest>,
) -> Result<Json<EvaluationResponse>, StatusCode> {
    // Parse FEN
    let position = EnginePosition::from_fen(&request.fen)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get evaluation
    let evaluation = crate::engine::evaluation::Evaluator::evaluate(&position);

    // Create description
    let description = if evaluation > 300 {
        "White has a significant advantage".to_string()
    } else if evaluation > 100 {
        "White has a slight advantage".to_string()
    } else if evaluation > -100 {
        "Position is roughly equal".to_string()
    } else if evaluation > -300 {
        "Black has a slight advantage".to_string()
    } else {
        "Black has a significant advantage".to_string()
    };

    Ok(Json(EvaluationResponse {
        evaluation,
        description,
    }))
}
