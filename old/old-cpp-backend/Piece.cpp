#include "Piece.h"

// Base Piece implementation
Piece::Piece(bool isWhite) : color(isWhite), numMoves(0) {}

void Piece::addMove() { numMoves++; }

int Piece::getNumMoves() const { return numMoves; }

bool Piece::getColor() const { return color; }

// Pawn implementation
Pawn::Pawn(bool isWhite) : Piece(isWhite) {}

std::string Pawn::toString() const { return color ? "♙" : "♟"; }

// Knight implementation
Knight::Knight(bool isWhite) : Piece(isWhite) {}

std::string Knight::toString() const { return color ? "♘" : "♞"; }

// Bishop implementation
Bishop::Bishop(bool isWhite) : Piece(isWhite) {}

std::string Bishop::toString() const { return color ? "♗" : "♝"; }

// Rook implementation
Rook::Rook(bool isWhite) : Piece(isWhite) {}

std::string Rook::toString() const { return color ? "♖" : "♜"; }

// Queen implementation
Queen::Queen(bool isWhite) : Piece(isWhite) {}

std::string Queen::toString() const { return color ? "♕" : "♛"; }

// King implementation
King::King(bool isWhite) : Piece(isWhite) {}

std::string King::toString() const { return color ? "♔" : "♚"; }

// EmptySquare implementation
EmptySquare::EmptySquare() : Piece(true) {}

std::string EmptySquare::toString() const { return " "; }

// #include "Piece.h"
// #include <string>
// #include <tuple>
// #include <vector>
//
// bool isWhite(pieceType piece) {
//   return piece >= WHITE_PAWN && piece <= WHITE_KING;
// }
//
// // Default constructor
// Piece::Piece() : piece(EMPTY), color(false) {}
// Piece::Piece(pieceType piece) {
//   this->piece = piece;
//   switch (piece) {
//   case WHITE_PAWN:
//   case WHITE_KNIGHT:
//   case WHITE_BISHOP:
//   case WHITE_ROOK:
//   case WHITE_QUEEN:
//   case WHITE_KING:
//     this->color = true;
//     break;
//   case BLACK_PAWN:
//   case BLACK_KNIGHT:
//   case BLACK_BISHOP:
//   case BLACK_ROOK:
//   case BLACK_QUEEN:
//   case BLACK_KING:
//     this->color = false;
//     break;
//   case EMPTY:
//   default:
//     this->color = false;
//     break;
//   }
//   // set number of moves to 0
//   this->numMoves = 0;
//
//   // function to check if a piece
// };
//
// void Piece::addMove() { numMoves++; }
//
// int Piece::getNumMoves() { return numMoves; }
//
// bool Piece::getColor() {
//   // add handling for empty piece
//   return color;
// }
// bool Piece::isSliding() {
//   return piece == WHITE_QUEEN || piece == WHITE_ROOK || piece == WHITE_BISHOP
//   ||
//          piece == BLACK_QUEEN || piece == BLACK_ROOK || piece ==
//          BLACK_BISHOP;
// }
// // checks if of the same type
//
// // type piece
// bool Piece::isRook() { return piece == WHITE_ROOK || piece == BLACK_ROOK; }
// bool Piece::isBishop() {
//   return piece == WHITE_BISHOP || piece == BLACK_BISHOP;
// }
// bool Piece::isQueen() { return piece == WHITE_QUEEN || piece == BLACK_QUEEN;
// } bool Piece::isKing() { return piece == WHITE_KING || piece == BLACK_KING; }
// bool Piece::isKnight() {
//   return piece == WHITE_KNIGHT || piece == BLACK_KNIGHT;
// }
// bool Piece::isPawn() { return piece == WHITE_PAWN || piece == BLACK_PAWN; }
// bool Piece::isEmpty() { return piece == EMPTY; }
//
// // type comparison
// bool Piece::isSameType(Piece piece) { return this->piece == piece.piece; }
//
// std::string Piece::toString() const {
//   switch (piece) {
//   case EMPTY:
//     return " ";
//   case WHITE_PAWN:
//     return "P";
//   case WHITE_KNIGHT:
//     return "N";
//   case WHITE_BISHOP:
//     return "B";
//   case WHITE_ROOK:
//     return "R";
//   case WHITE_QUEEN:
//     return "Q";
//   case WHITE_KING:
//     return "K";
//   case BLACK_PAWN:
//     return "p";
//   case BLACK_KNIGHT:
//     return "n";
//   case BLACK_BISHOP:
//     return "b";
//   case BLACK_ROOK:
//     return "r";
//   case BLACK_QUEEN:
//     return "q";
//   case BLACK_KING:
//     return "k";
//   default:
//     return "?";
//   }
// }
//
// Move::Move(int from, int to) : from(from), to(to) {}
// int Move::getFrom() { return from; }
// int Move::getTo() { return to; }
