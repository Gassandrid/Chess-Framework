/**
 * Chess Engine API Client
 * Provides functions to interact with the Rust chess engine backend
 */

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001';

export interface EngineInfo {
  id: string;
  name: string;
  version: string;
  description: string;
}

export interface TopMove {
  mv: string;
  score: number;
}

export interface TacticalFeatures {
  checks_available: number;
  captures_available: number;
  threatened_pieces: string[];
  hanging_pieces: string[];
}

export interface PositionalFeatures {
  material_balance: number;
  piece_activity: number;
  pawn_structure_score: number;
  king_safety: number;
  center_control: number;
}

export interface EvaluationBreakdown {
  classical_score: number;
  nn_score: number | null;
  deep_nn_score: number | null;
  final_score: number;
}

export interface ComprehensiveAnalysis {
  static_eval: number;
  search_eval: number | null;
  best_move: string | null;
  top_moves: TopMove[];
  evaluation_breakdown: EvaluationBreakdown | null;
  tactical_features: TacticalFeatures;
  positional_features: PositionalFeatures;
}

export interface MoveQuality {
  chosen_move: string;
  chosen_eval: number;
  best_move: string | null;
  best_eval: number;
  eval_loss: number;
  classification: 'Excellent' | 'Good' | 'Inaccuracy' | 'Mistake' | 'Blunder';
}

export interface BestMoveResponse {
  best_move: string;
  score: number | null;
  nodes: number;
  time_ms: number;
  nps: number;
}

/**
 * Get list of available engines
 */
export async function listEngines(): Promise<EngineInfo[]> {
  const response = await fetch(`${API_BASE_URL}/api/engine/list`);
  if (!response.ok) {
    throw new Error('Failed to fetch engines');
  }
  const data = await response.json();
  return data.engines;
}

/**
 * Get best move from a specific engine version
 */
export async function getBestMove(
  fen: string,
  engineVersion: string = 'v3',
  depth?: number,
  timeMs?: number
): Promise<BestMoveResponse> {
  const response = await fetch(`${API_BASE_URL}/api/engine/best-move-versioned`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      fen,
      engine_version: engineVersion,
      depth,
      time_ms: timeMs,
    }),
  });

  if (!response.ok) {
    throw new Error('Failed to get best move');
  }

  return response.json();
}

/**
 * Get comprehensive position analysis
 */
export async function analyzePosition(
  fen: string,
  depth?: number
): Promise<ComprehensiveAnalysis> {
  const response = await fetch(`${API_BASE_URL}/api/engine/comprehensive-analysis`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      fen,
      depth: depth || 8,
    }),
  });

  if (!response.ok) {
    throw new Error('Failed to analyze position');
  }

  return response.json();
}

/**
 * Evaluate the quality of a specific move
 */
export async function evaluateMoveQuality(
  fen: string,
  moveUci: string,
  depth?: number
): Promise<MoveQuality> {
  const response = await fetch(`${API_BASE_URL}/api/engine/move-quality`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      fen,
      move_uci: moveUci,
      depth: depth || 8,
    }),
  });

  if (!response.ok) {
    throw new Error('Failed to evaluate move quality');
  }

  return response.json();
}

/**
 * Convert board state to FEN notation
 * This is a simplified FEN converter - for full games you'd need castling rights, en passant, etc.
 */
export function boardToFEN(
  board: (string | null)[][],
  currentPlayer: 'white' | 'black'
): string {
  let fen = '';

  // Convert board to FEN piece placement
  for (let row = 0; row < 8; row++) {
    let emptyCount = 0;

    for (let col = 0; col < 8; col++) {
      const piece = board[row][col];

      if (piece === null) {
        emptyCount++;
      } else {
        if (emptyCount > 0) {
          fen += emptyCount;
          emptyCount = 0;
        }
        fen += piece;
      }
    }

    if (emptyCount > 0) {
      fen += emptyCount;
    }

    if (row < 7) {
      fen += '/';
    }
  }

  // Add active color
  fen += currentPlayer === 'white' ? ' w' : ' b';

  // Add default castling rights, en passant, halfmove, fullmove
  fen += ' KQkq - 0 1';

  return fen;
}

/**
 * Format evaluation score for display
 */
export function formatEvaluation(score: number): string {
  const absScore = Math.abs(score);

  if (absScore >= 1000) {
    // Mate in N
    const mateIn = Math.ceil((10000 - absScore) / 2);
    return score > 0 ? `M${mateIn}` : `-M${mateIn}`;
  }

  // Centipawn score
  const pawnValue = (score / 100).toFixed(1);
  return score > 0 ? `+${pawnValue}` : pawnValue;
}

/**
 * Get color for evaluation display
 */
export function getEvaluationColor(score: number): string {
  if (score > 100) return 'text-green-600';
  if (score > 0) return 'text-green-500';
  if (score < -100) return 'text-red-600';
  if (score < 0) return 'text-red-500';
  return 'text-gray-600';
}

/**
 * Get color for move quality classification
 */
export function getMoveQualityColor(classification: string): string {
  switch (classification) {
    case 'Excellent':
      return 'text-green-600';
    case 'Good':
      return 'text-green-500';
    case 'Inaccuracy':
      return 'text-yellow-500';
    case 'Mistake':
      return 'text-orange-500';
    case 'Blunder':
      return 'text-red-600';
    default:
      return 'text-gray-600';
  }
}
