#include "Game.h"

Game::Game() {
  this->board = Board(true);
  this->moves = std::vector<Move>();
}

Board &Game::getBoard() { return board; }

std::vector<Move> Game::generateMoves() {
  std::vector<Move> newMoves;

  for (int start = 0; start < 64; start++) {
    Piece piece = board.getPiece(start);
    if (!piece.isEmpty() && piece.getColor() == board.getWhiteToMove()) {
      // for sliding pieces
      if (piece.isSliding()) {
        generateSlidingMoves(start, piece);
      }

      // for pawns
      if (piece.isPawn()) {
        generatePawnMoves(start, piece);
      }

      // for knights
      if (piece.isKnight()) {
        generateKnightMoves(start, piece);
      }

      // for king
      if (piece.isKing()) {
        generateKingMoves(start, piece);
      }
    }
  }
  return newMoves;
}

void Game::generateKingMoves(int start, Piece piece) {
  // 8 directions, loop through each direction
  for (int direction = 0; direction < 8; direction++) {
    int target = start + board.getDirections(direction);
    Piece targetPiece = board.getPiece(target);

    // check if blocked by own piece
    if (!targetPiece.isEmpty() &&
        targetPiece.getColor() == board.getWhiteToMove()) {
      continue;
    }

    moves.push_back(Move(start, target));
  }
}

void Game::generatePawnMoves(int start, Piece piece) {
  int direction = (piece.getColor()) ? 8 : -8;
  int startingRank = (piece.getColor()) ? 1 : 6;
  int currentRank = start / 8;

  // Single move forward
  int target = start + direction;
  if (board.getPiece(target).isEmpty()) {
    moves.push_back(Move(start, target));

    // Double move if on starting rank
    if (currentRank == startingRank) {
      target = start + (direction * 2);
      if (board.getPiece(target).isEmpty()) {
        moves.push_back(Move(start, target));
      }
    }
  }

  // Captures
  int captureDirections[] = {direction - 1, direction + 1};
  for (int captureDir : captureDirections) {
    // Check if capture target is on same rank after move
    if ((start + captureDir) / 8 == (start + direction) / 8) {
      target = start + captureDir;
      if (!board.getPiece(target).isEmpty() &&
          board.getPiece(target).getColor() != board.getWhiteToMove()) {
        moves.push_back(Move(start, target));
      }
    }
  }
}

void Game::generateKnightMoves(int start, Piece piece) {
  // use the direction array for knights
  // array<int, 8> knightDirections = {15, 17, -15, -17, 6, 10, -6, -10};

  // go throhugh each direction, but make sure its not on an "edge" of the
  // board
  int getNumberSquaresToEdgeNorth = board.getNumberSquaresToEdge(start, 0);
  int getNumberSquaresToEdgeSouth = board.getNumberSquaresToEdge(start, 1);
  int getNumberSquaresToEdgeWest = board.getNumberSquaresToEdge(start, 2);
  int getNumberSquaresToEdgeEast = board.getNumberSquaresToEdge(start, 3);
  // if it has at least 2 squares to the edge, it can move in that direction
  if (getNumberSquaresToEdgeNorth > 1) {
    int target = start + board.getKnightDirections(0);
    Piece targetPiece = board.getPiece(target);
    if (targetPiece.isEmpty() ||
        targetPiece.getColor() != board.getWhiteToMove()) {
      moves.push_back(Move(start, target));
    }
  }
  if (getNumberSquaresToEdgeSouth > 1) {
    int target = start + board.getKnightDirections(1);
    Piece targetPiece = board.getPiece(target);
    if (targetPiece.isEmpty() ||
        targetPiece.getColor() != board.getWhiteToMove()) {
      moves.push_back(Move(start, target));
    }
  }
  if (getNumberSquaresToEdgeWest > 1) {
    int target = start + board.getKnightDirections(2);
    Piece targetPiece = board.getPiece(target);
    if (targetPiece.isEmpty() ||
        targetPiece.getColor() != board.getWhiteToMove()) {
      moves.push_back(Move(start, target));
    }
  }
  if (getNumberSquaresToEdgeEast > 1) {
    int target = start + board.getKnightDirections(3);
    Piece targetPiece = board.getPiece(target);
    if (targetPiece.isEmpty() ||
        targetPiece.getColor() != board.getWhiteToMove()) {
      moves.push_back(Move(start, target));
    }
  }
}

void Game::generateSlidingMoves(int start, Piece piece) {
  // 8 directions, loop through each direction
  int startDirectionIndex = (piece.isBishop()) ? 4 : 0;
  int endDirectionIndex = (piece.isRook()) ? 4 : 8;

  for (int direction = startDirectionIndex; direction < endDirectionIndex;
       direction++) {
    for (int i = 0; i < board.getNumberSquaresToEdge(start, direction); i++) {
      int target = start + board.getDirections(direction) * (i + 1);
      Piece targetPiece = board.getPiece(target);

      // check if blocked by own piece
      if (!targetPiece.isEmpty() &&
          targetPiece.getColor() == board.getWhiteToMove()) {
        break;
      }

      moves.push_back(Move(start, target));

      // if piece on target square is opponent color, break
      // Don't break on empty squares, only on opponent pieces
      if (!targetPiece.isEmpty() &&
          targetPiece.getColor() == board.getOpponentColor()) {
        break;
      }
    }
  }
}

// move validation for board move function
// takes a string move and just validates that it is a valid move
bool Game::validateMove(std::string move) {
  // make sure move is 2 characters long
  if (move.length() != 2) {
    return false;
  }

  // make sure first character is a letter between a-h
  // make sure second character is a number between 1-8
  char startFile = move[0];
  int startRank = move[1];
  if (startFile < 'a' || startFile > 'h' || startRank < 1 || startRank > 8) {
    return false;
  }
  return true;
}
