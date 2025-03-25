#ifndef BOARD_H
#define BOARD_H

#include "Piece.h"
#include <array>
#include <iostream>
#include <map>
#include <tuple>
#include <vector>

using namespace std;

/**
 * @brief representation for a chess board and its pieces
 *
 * maintains the state of the game, keeps track of current player and moves
 * also provides important data structures for move generation like number of
 * squares to edge
 *
 */
class Board {
private:
  /** @brief Array representing 64 squares of chess board */
  Piece board[64];

  /** @brief a collection of white pices currently on the board */
  vector<Piece> whitePieces;

  /** @brief prec alculated number of squares edge in each direction for all
   * squares squares */
  int NumberSquaresToEdge[64][8];

  /** @brief keeps track of whose turn it is based on if its white's turn */
  bool whiteToMove = true;

  /** @brief helper array for moving along a single dimesnion arary board
   *  Order: north, south, west, east, northwest, southeast, northeast,
   * southwest
   */
  array<int, 8> directions = {8, -8, -1, 1, 7, -7, 9, -9};

  /** @brief another helper array just for the knight
  array<int, 8> knightDirections = {15, 17, -15, -17, 6, 10, -6, -10};

  /** @brief Player's color
   * this will replace the playerColor variable in the future, but not used now
   * (true for white, false for black) */
  bool playerColor = true;

  /** @brief Current turn's color (true for white, false for black) */
  bool colorToMove = true;

public:
  /**
   * @brief constructor for the boarrd given a player color
   * @param playerColor The color for the player (true for white, false for
   * black)
   */
  Board(bool playerColor = true);

  /**
   * @brief Constructs a board from a FEN (forsyth–edwards notation) string
   * @param fen The FEN string representing the board position
   */
  Board(string fen);

  /**
   * @brief validation helper for the move
   * does not actually move the piece, and this will be replaced by legal move
   * calulator
   * @param to Destination square in algebraic notation (e.g., "e4")
   * @param from Starting square in algebraic notation
   * @return true if the move is valid and was executed, false otherwise
   */
  bool validateAndMovePiece(string to, string from);

  /**
   * @brief craetes the lookup table for finding nearby edges to a certain
   * square
   */
  void initNumberSquaresToEdge();

  /**
   * @brief output current board state to console
   */
  void printBoard();

  /**
   * @brief converts a coordainte to an index
   * @param move Chess square in algebraic notation: "e4"
   * @return array index from 0 to 63
   */
  int convertMove(string move);

  /**
   * @brief converts index to a coordinate ( not used currently )
   * @param index index from 0 to 63
   * @return a chess square in algebraic notation: "e4"
   */
  string convertMove(int index);

  /**
   * @brief for tracking turn, checks if whites turn
   * @return true if white to move, false if black
   */
  bool getWhiteToMove();

  /**
   * @brief gets the color of the current turn
   * this will replace the playerColor variable in the future, but not used now
   * @return true for white's turn, false for black's turn
   */
  bool getColorToMove();

  /**
   * @brief Gets the piece at specified board position
   * @param index Board position index from 0 to 63
   * @return Piece object at the specified position
   */
  Piece getPiece(int index);

  /**
   * @brief for using the pre computed data for edges and such
   * @param index Starting square index 0 to 63
   * @param direction Direction index 0 to 7
   * @return Number of squares to edge in specified direction
   */
  int getNumberSquaresToEdge(int index, int direction);

  /**
   * @brief direction lookup for the normal directions
   * @param index Direction index 0 to 7
   * @return Integer offset for the direction
   */
  int getDirections(int index);

  /**
   * @brief direction lookup for the knight directions
   * @param index Knight move index (0-7)
   * @return Integer offset for the knight move
   */
  int getKnightDirections(int index);

  /**
   * @brief the mover function for the board
   * does not have any validation, meant to be used by the game class
   * @param from Starting square index (0-63)
   * @param to Destination square index (0-63)
   */
  void movePiece(int from, int to);

  /**
   * @brief get player's color
   * @return 1 for white, 0 for black
   */
  int getPlayerColor();

  /**
   * @brief get opponent's color
   * @return 1 for white, 0 for black
   */
  int getOpponentColor();
};

#endif // BOARD_H
