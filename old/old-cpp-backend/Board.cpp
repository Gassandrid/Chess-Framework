// for defining the printed board system and how to render pieces.
#include "Board.h"
#include "Piece.h"
#include <array>
#include <iostream>
#include <map>
#include <tuple>
#include <vector>

using namespace std;

// default constructor loadds the board with pieces
Board::Board(bool playerColor) {
  this->playerColor = playerColor;
  whiteToMove = true;
  colorToMove = true;
  // fill with empty first
  for (int i = 0; i < 64; i++) {
    board[i] = Piece(EMPTY);
  }

  // white pieces
  board[0] = Piece(WHITE_ROOK);
  board[1] = Piece(WHITE_KNIGHT);
  board[2] = Piece(WHITE_BISHOP);
  board[3] = Piece(WHITE_QUEEN);
  board[4] = Piece(WHITE_KING);
  board[5] = Piece(WHITE_BISHOP);
  board[6] = Piece(WHITE_KNIGHT);
  board[7] = Piece(WHITE_ROOK);
  for (int i = 8; i < 16; i++) {
    board[i] = Piece(WHITE_PAWN);
  }

  // black pieces
  for (int i = 48; i < 56; i++) {
    board[i] = Piece(BLACK_PAWN);
  }
  board[56] = Piece(BLACK_ROOK);
  board[57] = Piece(BLACK_KNIGHT);
  board[58] = Piece(BLACK_BISHOP);
  board[59] = Piece(BLACK_QUEEN);
  board[60] = Piece(BLACK_KING);
  board[61] = Piece(BLACK_BISHOP);
  board[62] = Piece(BLACK_KNIGHT);
  board[63] = Piece(BLACK_ROOK);

  // calculate the number of squares to edge
  initNumberSquaresToEdge();
}

// load from fen string
Board::Board(string fen) {
  whiteToMove = true;
  colorToMove = true;
  int rank = 7; // start from top rank (8th rank)
  int file = 0; // start from a-file

  // Clear the board first
  for (int i = 0; i < 64; i++) {
    board[i] = Piece(EMPTY);
  }

  for (char c : fen) {
    if (c == '/') {
      rank--;
      file = 0;
    } else if (isdigit(c)) {
      file += (c - '0'); // skip empty squares
    } else {
      int index = rank * 8 + file;
      // Place the piece
      switch (c) {
      case 'P':
        board[index] = Piece(WHITE_PAWN);
        break;
      case 'N':
        board[index] = Piece(WHITE_KNIGHT);
        break;
      case 'B':
        board[index] = Piece(WHITE_BISHOP);
        break;
      case 'R':
        board[index] = Piece(WHITE_ROOK);
        break;
      case 'Q':
        board[index] = Piece(WHITE_QUEEN);
        break;
      case 'K':
        board[index] = Piece(WHITE_KING);
        break;
      case 'p':
        board[index] = Piece(BLACK_PAWN);
        break;
      case 'n':
        board[index] = Piece(BLACK_KNIGHT);
        break;
      case 'b':
        board[index] = Piece(BLACK_BISHOP);
        break;
      case 'r':
        board[index] = Piece(BLACK_ROOK);
        break;
      case 'q':
        board[index] = Piece(BLACK_QUEEN);
        break;
      case 'k':
        board[index] = Piece(BLACK_KING);
        break;
      }
      file++;
    }
    if (c == ' ')
      break;
  }

  // calculate the number of squares to edge
  initNumberSquaresToEdge();
}

// populating the number of squares to edge array
void Board::initNumberSquaresToEdge() {
  for (int rank = 0; rank < 8; rank++) {
    for (int file = 0; file < 8; file++) {
      int north = 7 - rank;
      int south = rank;
      int west = file;
      int east = 7 - file;

      int squareIndex = rank * 8 + file;

      // Store the values for each direction
      NumberSquaresToEdge[squareIndex][0] = north;
      NumberSquaresToEdge[squareIndex][1] = south;
      NumberSquaresToEdge[squareIndex][2] = west;
      NumberSquaresToEdge[squareIndex][3] = east;
      NumberSquaresToEdge[squareIndex][4] = std::min(north, west);
      NumberSquaresToEdge[squareIndex][5] = std::min(north, east);
      NumberSquaresToEdge[squareIndex][6] = std::min(south, west);
      NumberSquaresToEdge[squareIndex][7] = std::min(south, east);
    }
  }
}

bool Board::validateAndMovePiece(string to, string from) {
  // first make sure both are 2 characters long strings, first is letter a-h and
  // second is number 1-8
  if (to.length() != 2 || from.length() != 2) {
    cout << "Invalid move, please enter a move two chatacters long, a-b and 1-8"
         << endl;
    return false;
  }

  if (to[0] < 'a' || to[0] > 'h' || from[0] < 'a' || from[0] > 'h') {
    cout << "Invalid move, please enter a move two chatacters long, a-b and 1-8"
         << endl;
    return false;
  }

  if (to[1] < '1' || to[1] > '8' || from[1] < '1' || from[1] > '8') {
    cout << "Invalid move, please enter a move two chatacters long, a-b and 1-8"
         << endl;
    return false;
  }

  // if these pass, then the move is pseudo legal
  // this function will be replaced by the legal move generator later on
  return true;
}

// move piece from one square to another
// this is not validated, game class handles that
void Board::movePiece(int from, int to) {
  // Store the piece we're moving
  Piece movingPiece = board[from];
  // Clear the original position
  board[from] = Piece(EMPTY);
  // Place the stored piece in the new position
  board[to] = movingPiece;
  // Add move to the piece's counter
  board[to].addMove();
  // Toggle turns
  whiteToMove = !whiteToMove;
  colorToMove = !colorToMove;
}

void Board::printBoard() {
  cout << "  a b c d e f g h" << endl; // file labels on top
  cout << "  ---------------" << endl;
  for (int i = 7; i >= 0; i--) { // print ranks in reverse order (8 to 1)
    cout << (i + 1) << "|";      // rank number
    for (int j = 0; j < 8; j++) {
      cout << board[i * 8 + j].toString() << " ";
    }
    cout << "|" << (i + 1) << endl; // rank number on right side
  }
  cout << "  ---------------" << endl;
  cout << "  a b c d e f g h" << endl; // file labels on bottom
}

// converter function from chess coordinates to array index
int Board::convertMove(string move) {
  int file = move[0] - 'a';
  int rank = move[1] - '1';
  return rank * 8 + file;
}

// converter function from array indices to chess coordinates
string Board::convertMove(int index) {
  string move = "";
  move += (char)(index % 8 + 'a');
  move += (char)(index / 8 + '1');
  return move;
}

bool Board::getWhiteToMove() { return whiteToMove; }
bool Board::getColorToMove() { return colorToMove; }
Piece Board::getPiece(int index) { return board[index]; }

// get NumberSquaresToEdge
int Board::getNumberSquaresToEdge(int index, int direction) {
  return NumberSquaresToEdge[index][direction];
}

int Board::getDirections(int index) { return directions[index]; }
int Board::getKnightDirections(int index) { return knightDirections[index]; }

int Board::getPlayerColor() { return playerColor; }
int Board::getOpponentColor() { return !playerColor; }
