#ifndef PIECE_H
#define PIECE_H

#include <string>
#include <vector>

/**
 * @brief Abstract base class representing a chess piece
 */
class Piece {
protected:
  bool color;   // white is true, black is false
  int numMoves; // number of moves a piece has made

public:
  /**
   * @brief Constructs a piece with the given color
   * @param isWhite true for white piece, false for black
   */
  Piece(bool isWhite);

  /**
   * @brief Virtual destructor
   */
  virtual ~Piece() = default;

  /**
   * @brief Increments move counter for piece
   */
  void addMove();

  /**
   * @brief Gets number of moves piece has made
   * @return The number of moves
   */
  int getNumMoves() const;

  /**
   * @brief Gets the color of the piece
   * @return true for white false for black
   */
  bool getColor() const;

  /**
   * @brief Checks if this piece can slide (bishop, rook, queen)
   * @return true if the piece can slide, false otherwise
   */
  virtual bool isSliding() const = 0;

  /**
   * @brief Checks if this piece is a pawn
   * @return true if the piece is a pawn, false otherwise
   */
  virtual bool isPawn() const { return false; }

  /**
   * @brief Checks if this piece is a knight
   * @return true if the piece is a knight, false otherwise
   */
  virtual bool isKnight() const { return false; }

  /**
   * @brief Checks if this piece is a bishop
   * @return true if the piece is a bishop, false otherwise
   */
  virtual bool isBishop() const { return false; }

  /**
   * @brief Checks if this piece is a rook
   * @return true if the piece is a rook, false otherwise
   */
  virtual bool isRook() const { return false; }

  /**
   * @brief Checks if this piece is a queen
   * @return true if the piece is a queen, false otherwise
   */
  virtual bool isQueen() const { return false; }

  /**
   * @brief Checks if this piece is a king
   * @return true if the piece is a king, false otherwise
   */
  virtual bool isKing() const { return false; }

  /**
   * @brief Checks if this is an empty square
   * @return true if this is an empty square, false otherwise
   */
  virtual bool isEmpty() const { return false; }

  /**
   * @brief Gets the symbol representation of the piece
   * @return String representation of the piece
   */
  virtual std::string toString() const = 0;
};

/**
 * @brief Represents a Pawn chess piece
 */
class Pawn : public Piece {
public:
  explicit Pawn(bool isWhite);
  bool isSliding() const override { return false; }
  bool isPawn() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents a Knight chess piece
 */
class Knight : public Piece {
public:
  explicit Knight(bool isWhite);
  bool isSliding() const override { return false; }
  bool isKnight() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents a Bishop chess piece
 */
class Bishop : public Piece {
public:
  explicit Bishop(bool isWhite);
  bool isSliding() const override { return true; }
  bool isBishop() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents a Rook chess piece
 */
class Rook : public Piece {
public:
  explicit Rook(bool isWhite);
  bool isSliding() const override { return true; }
  bool isRook() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents a Queen chess piece
 */
class Queen : public Piece {
public:
  explicit Queen(bool isWhite);
  bool isSliding() const override { return true; }
  bool isQueen() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents a King chess piece
 */
class King : public Piece {
public:
  explicit King(bool isWhite);
  bool isSliding() const override { return false; }
  bool isKing() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents an empty square
 */
class EmptySquare : public Piece {
public:
  EmptySquare();
  bool isSliding() const override { return false; }
  bool isEmpty() const override { return true; }
  std::string toString() const override;
};

/**
 * @brief Represents a move from one square to another
 */
class Move {
private:
  int from; ///< Starting square index
  int to;   ///< Destination square index

public:
  /**
   * @brief Constructs a move
   * @param from Starting square index
   * @param to Destination square index
   */
  Move(int from, int to);

  /**
   * @brief Gets the starting square
   * @return Index of starting square
   */
  int getFrom();

  /**
   * @brief Gets the destination square
   * @return Index of destination square
   */
  int getTo();
};

#endif // PIECE_H
