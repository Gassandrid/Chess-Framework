"use client";

import { cn } from "@/lib/utils";
import { Loader2 } from "lucide-react";

// typescript types for the chess board
// this is not the same as the backend types, but it will do
type PieceType =
  | "p"
  | "r"
  | "n"
  | "b"
  | "q"
  | "k"
  | "P"
  | "R"
  | "N"
  | "B"
  | "Q"
  | "K"
  | null;
type BoardState = PieceType[][];
type Square = { row: number; col: number };
type Player = "white" | "black";

// main  componnent declaratino
interface ChessBoardProps {
  boardState: BoardState;
  selectedSquare: Square | null;
  possibleMoves: Square[];
  onSquareClick: (square: Square) => void;
  currentPlayer: Player;
  isLoading: boolean;
}

// implementation of the chess board component
export default function ChessBoard({
  boardState,
  selectedSquare,
  possibleMoves,
  onSquareClick,
  currentPlayer,
  isLoading,
}: ChessBoardProps) {
  // i am lazy, i dont want to find graphics for
  // the pieces and will just use unicode instead lol
  const pieceToUnicode: Record<string, string> = {
    k: "♚",
    q: "♛",
    r: "♜",
    b: "♝",
    n: "♞",
    p: "♟",
    K: "♔",
    Q: "♕",
    R: "♖",
    B: "♗",
    N: "♘",
    P: "♙",
  };

  // NOTE THIS FUNCTION IS NOT IMPLEMENTED - WE WILL USE BACKEND FOR THIS
  // to determine if a move is possible
  const isPossibleMove = (row: number, col: number) => {
    return possibleMoves.some((move) => move.row === row && move.col === col);
  };

  // check if square is selected
  const isSelected = (row: number, col: number) => {
    return selectedSquare?.row === row && selectedSquare?.col === col;
  };

  // square color getter
  const getSquareColor = (row: number, col: number) => {
    const isLight = (row + col) % 2 === 0;
    return isLight
      ? "bg-card hover:bg-muted"
      : "bg-muted hover:bg-muted/80 dark:bg-muted/80 dark:hover:bg-muted/60";
  };

  // piece color getter
  const getPieceColor = (piece: PieceType) => {
    if (!piece) return "";
    return piece === piece.toUpperCase()
      ? "text-primary"
      : "text-primary-foreground dark:text-primary";
  };

  // the html return for this component
  return (
    <div className="relative w-full aspect-square max-w-[600px] mx-auto">
      {isLoading && (
        <div className="absolute inset-0 bg-background/80 flex items-center justify-center z-50 rounded-md">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      )}

      <div className="grid grid-cols-8 h-full w-full border border-border rounded-md overflow-hidden">
        {boardState.map((row, rowIndex) =>
          row.map((piece, colIndex) => (
            <div
              key={`${rowIndex}-${colIndex}`}
              className={cn(
                "relative flex items-center justify-center cursor-pointer transition-all",
                "h-full w-full aspect-square",
                getSquareColor(rowIndex, colIndex),
                isSelected(rowIndex, colIndex) &&
                  "ring-2 ring-inset ring-primary",
                isPossibleMove(rowIndex, colIndex) &&
                  "after:absolute after:inset-0 after:bg-primary/20",
              )}
              onClick={() => onSquareClick({ row: rowIndex, col: colIndex })}
            >
              <div className="flex items-center justify-center w-full h-full">
                {piece && (
                  <span
                    className={cn("text-4xl select-none", getPieceColor(piece))}
                  >
                    {pieceToUnicode[piece]}
                  </span>
                )}
              </div>

              {/* click action stuff to show possible moves */}
              {isPossibleMove(rowIndex, colIndex) && !piece && (
                <div className="absolute w-3 h-3 rounded-full bg-primary/50"></div>
              )}

              {colIndex === 0 && (
                <span className="absolute left-1 top-1 text-xs opacity-70">
                  {8 - rowIndex}
                </span>
              )}
              {rowIndex === 7 && (
                <span className="absolute right-1 bottom-1 text-xs opacity-70">
                  {String.fromCharCode(97 + colIndex)}
                </span>
              )}
            </div>
          )),
        )}
      </div>
    </div>
  );
}
