// #include "Game.h"
#include "Board.h"
#include <iostream>
#include <string>

using namespace std;

int main() {

  try {
    // initialize the board
    Board board;

    // give initial instructions
    cout << "welcome to chess" << endl;
    cout << "enter a move as the form of a initial square and a target square, "
            "you will be prompted"
         << endl;
    cout << "for example, to move a pawn from a2 to a4, type 'a2' for first "
            "prompt and 'a4' for second"
         << endl;

    cout << "type q to quit" << endl;

    // main game loop
    while (true) {
      cout << "----------------------------------------------------------------"
              "--"
           << endl;

      cout << "Current board:" << endl;
      board.printBoard();

      string target, destination;

      // debug: white should go first
      // cout << "whiteToMove value: " << board.getWhiteToMove() << endl;

      cout << (board.getWhiteToMove() ? "White" : "Black") << " to move"
           << endl;

      cout << "Enter the target piece(q to quit): ";
      getline(cin, target);

      if (target == "q" || target == "Q") {
        break;
      }

      cout << "Enter the destination(q to quit): ";
      getline(cin, destination);

      if (destination == "q" || destination == "Q") {
        break;
      }

      // move validation loop until player enters a valid move
      bool validMove = false;
      while (!validMove) {
        if (board.validateAndMovePiece(target, destination)) {
          cout << "Piece moved from " << target << " to " << destination
               << endl;

          board.movePiece(board.convertMove(target),
                          board.convertMove(destination));
          validMove = true;
        } else {
          cout << "Invalid move. Please try again." << endl;
          cout << "Enter the target piece(q to quit): ";
          getline(cin, target);

          if (target == "q" || target == "Q") {
            return 0;
          }

          cout << "Enter the destination(q to quit): ";
          getline(cin, destination);

          if (destination == "q" || destination == "Q") {
            return 0;
          }
        }
      }
    }
    // just in case catch all, should never get here
  } catch (const std::exception &e) {
    cerr << "Fatal error: " << e.what() << endl;
  }
}
