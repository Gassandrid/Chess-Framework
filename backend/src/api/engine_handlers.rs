use crate::engine::{
    perft::Perft,
    position::Position as EnginePosition,
    search::{Search, SearchLimits},
    analysis::{Analyzer, MoveClassification},
    ensemble_eval::EnsembleEvaluator,
    movegen::Move,
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

/// Comprehensive position analysis
#[derive(Debug, Deserialize)]
pub struct ComprehensiveAnalysisRequest {
    pub fen: String,
    pub depth: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct TacticalFeaturesResponse {
    pub checks_available: usize,
    pub captures_available: usize,
    pub threatened_pieces: Vec<String>,
    pub hanging_pieces: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PositionalFeaturesResponse {
    pub material_balance: i32,
    pub piece_activity: i32,
    pub pawn_structure_score: i32,
    pub king_safety: i32,
    pub center_control: i32,
}

#[derive(Debug, Serialize)]
pub struct EvaluationBreakdownResponse {
    pub classical_score: i32,
    pub nn_score: Option<i32>,
    pub deep_nn_score: Option<i32>,
    pub final_score: i32,
}

#[derive(Debug, Serialize)]
pub struct ComprehensiveAnalysisResponse {
    pub static_eval: i32,
    pub search_eval: Option<i32>,
    pub best_move: Option<String>,
    pub top_moves: Vec<TopMoveInfo>,
    pub evaluation_breakdown: Option<EvaluationBreakdownResponse>,
    pub tactical_features: TacticalFeaturesResponse,
    pub positional_features: PositionalFeaturesResponse,
}

#[derive(Debug, Serialize)]
pub struct TopMoveInfo {
    pub mv: String,
    pub score: i32,
}

pub async fn comprehensive_analysis(
    Json(request): Json<ComprehensiveAnalysisRequest>,
) -> Result<Json<ComprehensiveAnalysisResponse>, StatusCode> {
    let position = EnginePosition::from_fen(&request.fen)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let depth = request.depth.unwrap_or(8);
    let mut analyzer = Analyzer::new();

    let analysis = analyzer.analyze_position(&position, depth);

    let top_moves = analysis.top_moves.iter()
        .map(|(mv, score)| TopMoveInfo {
            mv: mv.to_uci(),
            score: *score,
        })
        .collect();

    let eval_breakdown = analysis.evaluation_breakdown.map(|eb| EvaluationBreakdownResponse {
        classical_score: eb.classical_score,
        nn_score: eb.nn_score,
        deep_nn_score: eb.deep_nn_score,
        final_score: eb.final_score,
    });

    Ok(Json(ComprehensiveAnalysisResponse {
        static_eval: analysis.static_eval,
        search_eval: analysis.search_eval,
        best_move: analysis.best_move.map(|m| m.to_uci()),
        top_moves,
        evaluation_breakdown: eval_breakdown,
        tactical_features: TacticalFeaturesResponse {
            checks_available: analysis.tactical_features.checks_available,
            captures_available: analysis.tactical_features.captures_available,
            threatened_pieces: analysis.tactical_features.threatened_pieces,
            hanging_pieces: analysis.tactical_features.hanging_pieces,
        },
        positional_features: PositionalFeaturesResponse {
            material_balance: analysis.positional_features.material_balance,
            piece_activity: analysis.positional_features.piece_activity,
            pawn_structure_score: analysis.positional_features.pawn_structure_score,
            king_safety: analysis.positional_features.king_safety,
            center_control: analysis.positional_features.center_control,
        },
    }))
}

/// Evaluate move quality
#[derive(Debug, Deserialize)]
pub struct MoveQualityRequest {
    pub fen: String,
    pub move_uci: String,
    pub depth: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct MoveQualityResponse {
    pub chosen_move: String,
    pub chosen_eval: i32,
    pub best_move: Option<String>,
    pub best_eval: i32,
    pub eval_loss: i32,
    pub classification: String,
}

pub async fn evaluate_move_quality(
    Json(request): Json<MoveQualityRequest>,
) -> Result<Json<MoveQualityResponse>, StatusCode> {
    let position = EnginePosition::from_fen(&request.fen)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let chosen_move = Move::from_uci(&request.move_uci, &position)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let depth = request.depth.unwrap_or(8);
    let mut analyzer = Analyzer::new();

    let quality = analyzer.analyze_move_quality(&position, chosen_move, depth);

    let classification = match quality.classification {
        MoveClassification::Excellent => "Excellent",
        MoveClassification::Good => "Good",
        MoveClassification::Inaccuracy => "Inaccuracy",
        MoveClassification::Mistake => "Mistake",
        MoveClassification::Blunder => "Blunder",
    };

    Ok(Json(MoveQualityResponse {
        chosen_move: quality.chosen_move.to_uci(),
        chosen_eval: quality.chosen_eval,
        best_move: quality.best_move.map(|m| m.to_uci()),
        best_eval: quality.best_eval,
        eval_loss: quality.eval_loss,
        classification: classification.to_string(),
    }))
}

/// Get list of available engines
#[derive(Debug, Serialize)]
pub struct EngineInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct EngineListResponse {
    pub engines: Vec<EngineInfo>,
}

pub async fn list_engines() -> Json<EngineListResponse> {
    Json(EngineListResponse {
        engines: vec![
            EngineInfo {
                id: "v1".to_string(),
                name: "Chess Engine V1.0".to_string(),
                version: "1.0".to_string(),
                description: "Baseline engine with basic alpha-beta search".to_string(),
            },
            EngineInfo {
                id: "v2".to_string(),
                name: "Chess Engine V2.0".to_string(),
                version: "2.0".to_string(),
                description: "Improved evaluation, MVV-LVA ordering, null move pruning".to_string(),
            },
            EngineInfo {
                id: "v3".to_string(),
                name: "Chess Engine V3.0".to_string(),
                version: "3.0".to_string(),
                description: "Advanced search with aspiration windows, razoring, futility pruning".to_string(),
            },
        ],
    })
}

/// Get best move from specific engine version
#[derive(Debug, Deserialize)]
pub struct EngineVersionMoveRequest {
    pub fen: String,
    pub engine_version: String,
    pub depth: Option<u8>,
    pub time_ms: Option<u64>,
}

pub async fn get_best_move_versioned(
    Json(request): Json<EngineVersionMoveRequest>,
) -> Result<Json<EngineMoveResponse>, StatusCode> {
    let position = EnginePosition::from_fen(&request.fen)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut limits = SearchLimits::default();
    if let Some(depth) = request.depth {
        limits.max_depth = Some(depth);
    }
    if let Some(time) = request.time_ms {
        limits.max_time = Some(Duration::from_millis(time));
    }

    let (best_move, nodes, time_ms) = match request.engine_version.as_str() {
        "v1" => {
            let mut search = crate::engine::search::Search::new(128);
            let limits_v1 = crate::engine::search::SearchLimits {
                max_depth: limits.max_depth,
                max_time: limits.max_time,
                ..Default::default()
            };
            let mv = search.search(&position, limits_v1);
            let stats = search.get_stats();
            (mv, stats.nodes + stats.qnodes, stats.time_elapsed.as_millis())
        }
        "v2" => {
            let mut search = crate::engine::search_v2::SearchV2::new(128);
            let limits_v2 = crate::engine::search_v2::SearchLimits {
                max_depth: limits.max_depth,
                max_time: limits.max_time,
                ..Default::default()
            };
            let mv = search.search(&position, limits_v2);
            let stats = search.get_stats();
            (mv, stats.nodes + stats.qnodes, stats.time_elapsed.as_millis())
        }
        "v3" => {
            let mut search = crate::engine::search_v3::SearchV3::new(128);
            let limits_v3 = crate::engine::search_v3::SearchLimits {
                max_depth: limits.max_depth,
                max_time: limits.max_time,
                ..Default::default()
            };
            let mv = search.search(&position, limits_v3);
            // SearchV3 doesn't have get_stats, use placeholder values
            (mv, 0, 0)
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    if let Some(mv) = best_move {
        let nps = if time_ms > 0 {
            (nodes as f64 / (time_ms as f64 / 1000.0)) as u64
        } else {
            0
        };

        Ok(Json(EngineMoveResponse {
            best_move: mv.to_uci(),
            score: None,
            nodes,
            time_ms,
            nps,
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
