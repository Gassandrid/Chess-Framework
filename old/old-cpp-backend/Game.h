#ifndef GAME_H
#define GAME_H

#include "Board.h"
#include <vector>

/**
 * @brief for managing the game state and logic, eg. move generation and whatnot
 *
 * Handles the generation of all legal moves for each piece in the current
 * board class. Is a wrapper around the board class, but helps to manage its
 * functions like validation of moves to board vector
 *
 */
class Game {
private:
  /** @brief the board object for the game */
  Board board;

  /** @brief current list of legal moves */
  std::vector<Move> moves;

public:
  /**
   * @brief default constructor for the game to initialize the board
   */
  Game();

  /**
   * @brief gets a reference since the object can be quite large
   * @return Reference to the current Board object
   */
  Board &getBoard();

  /**
   * @brief Generates legal moves in current position
   * @return Vector containing all possible legal moves
   */
  std::vector<Move> generateMoves();

  /**
   * @brief generates all legal moves for a king
   * @param start Starting square index 0 to 63
   * @param piece The king piece to generate moves for
   */
  void generateKingMoves(int start, Piece piece);

  /**
   * @brief generates all legal moves for a pawn
   * @param start Starting square index 0 to 63
   * @param piece The pawn piece to generate moves for
   */
  void generatePawnMoves(int start, Piece piece);

  /**
   * @brief generates all legal moves for a knight
   * @param start Starting square index 0 to 63
   * @param piece The knight piece to generate moves for
   */
  void generateKnightMoves(int start, Piece piece);

  /**
   * @brief generates all legal moves for sliding pieces (queen, rook, bishop)
   * @param start Starting square index 0 to 63
   * @param piece The sliding piece to generate moves for
   */
  void generateSlidingMoves(int start, Piece piece);

  /**
   * @brief Validates if a proposed move is legal
   * not currently used but will manage board class move validation
   * @param move the move as a coordinate string, eg. "a2"
   * @return true if the move is legal, false otherwise
   */
  bool validateMove(std::string move);
};

#endif
