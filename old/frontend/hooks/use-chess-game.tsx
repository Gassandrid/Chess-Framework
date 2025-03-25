"use client";

import { useState, useCallback } from "react";

// Types for the chess game
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
type GameStatus = "active" | "check" | "checkmate" | "stalemate" | "draw";

interface Move {
  from: string;
  to: string;
  piece: string;
  capture?: string;
  promotion?: string;
  check?: boolean;
  checkmate?: boolean;
  notation: string;
}

// Initial board setup (standard chess starting position)
const initialBoardState: BoardState = [
  ["r", "n", "b", "q", "k", "b", "n", "r"],
  ["p", "p", "p", "p", "p", "p", "p", "p"],
  [null, null, null, null, null, null, null, null],
  [null, null, null, null, null, null, null, null],
  [null, null, null, null, null, null, null, null],
  [null, null, null, null, null, null, null, null],
  ["P", "P", "P", "P", "P", "P", "P", "P"],
  ["R", "N", "B", "Q", "K", "B", "N", "R"],
];

// Convert board coordinates to algebraic notation
const toAlgebraic = (square: Square): string => {
  const file = String.fromCharCode(97 + square.col);
  const rank = 8 - square.row;
  return `${file}${rank}`;
};

// This hook manages the chess game state and provides methods for interacting with the backend
export function useChessGame() {
  const [boardState, setBoardState] = useState<BoardState>(initialBoardState);
  const [currentPlayer, setCurrentPlayer] = useState<Player>("white");
  const [gameStatus, setGameStatus] = useState<GameStatus>("active");
  const [selectedSquare, setSelectedSquare] = useState<Square | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<Square[]>([]);
  const [moveHistory, setMoveHistory] = useState<Move[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Function to fetch legal moves from the backend
  const fetchLegalMoves = useCallback(
    async (square: Square) => {
      try {
        setIsLoading(true);
        setError(null);

        // This would be replaced with an actual API call to your backend
        // Example API call:
        // const response = await fetch('/api/chess/legal-moves', {
        //   method: 'POST',
        //   headers: { 'Content-Type': 'application/json' },
        //   body: JSON.stringify({
        //     position: boardState,
        //     square: toAlgebraic(square),
        //     currentPlayer
        //   })
        // })
        // const data = await response.json()
        // if (!response.ok) throw new Error(data.message)
        // return data.moves.map(move => ({ row: 8 - parseInt(move[1]), col: move.charCodeAt(0) - 97 }))

        // For demo purposes, we'll return some hardcoded moves based on the piece
        const piece = boardState[square.row][square.col];

        // Mock implementation - in a real app, this would come from the backend
        // This is just to demonstrate the UI flow
        const mockMoves: Square[] = [];

        // Pawn basic moves (very simplified, just for demo)
        if (piece === "P" && square.row > 0) {
          mockMoves.push({ row: square.row - 1, col: square.col });
          if (square.row === 6)
            mockMoves.push({ row: square.row - 2, col: square.col });
        } else if (piece === "p" && square.row < 7) {
          mockMoves.push({ row: square.row + 1, col: square.col });
          if (square.row === 1)
            mockMoves.push({ row: square.row + 2, col: square.col });
        }

        // Add some random moves for other pieces to demonstrate UI
        if (piece && piece.toLowerCase() !== "p") {
          for (let i = 0; i < 3; i++) {
            const randomRow = Math.floor(Math.random() * 8);
            const randomCol = Math.floor(Math.random() * 8);
            mockMoves.push({ row: randomRow, col: randomCol });
          }
        }

        return mockMoves;
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Failed to fetch legal moves",
        );
        return [];
      } finally {
        setIsLoading(false);
      }
    },
    [boardState, currentPlayer],
  );

  // Handle square click
  const handleSquareClick = useCallback(
    async (square: Square) => {
      const piece = boardState[square.row][square.col];

      // If no square is selected, select this square if it has a piece of the current player
      if (!selectedSquare) {
        if (piece) {
          const isPieceCurrentPlayer =
            (currentPlayer === "white" && piece === piece.toUpperCase()) ||
            (currentPlayer === "black" && piece === piece.toLowerCase());

          if (isPieceCurrentPlayer) {
            setSelectedSquare(square);
            const legalMoves = await fetchLegalMoves(square);
            setPossibleMoves(legalMoves);
          }
        }
        return;
      }

      // If a square is already selected

      // If clicking the same square, deselect it
      if (
        selectedSquare.row === square.row &&
        selectedSquare.col === square.col
      ) {
        setSelectedSquare(null);
        setPossibleMoves([]);
        return;
      }

      // Check if the clicked square is a valid move
      const isValidMove = possibleMoves.some(
        (move) => move.row === square.row && move.col === square.col,
      );

      if (isValidMove) {
        // Make the move
        makeMove(selectedSquare, square);
      } else {
        // If clicking another piece of the same color, select that piece instead
        const newPiece = boardState[square.row][square.col];
        const isPieceCurrentPlayer =
          (currentPlayer === "white" &&
            newPiece &&
            newPiece === newPiece.toUpperCase()) ||
          (currentPlayer === "black" &&
            newPiece &&
            newPiece === newPiece.toLowerCase());

        if (isPieceCurrentPlayer) {
          setSelectedSquare(square);
          const legalMoves = await fetchLegalMoves(square);
          setPossibleMoves(legalMoves);
        } else {
          // If clicking an empty square or opponent's piece that's not a valid move
          setSelectedSquare(null);
          setPossibleMoves([]);
        }
      }
    },
    [boardState, currentPlayer, selectedSquare, possibleMoves, fetchLegalMoves],
  );

  // Make a move
  const makeMove = useCallback(
    async (from: Square, to: Square) => {
      try {
        setIsLoading(true);
        setError(null);

        // This would be replaced with an actual API call to your backend
        // Example API call:
        // const response = await fetch('/api/chess/make-move', {
        //   method: 'POST',
        //   headers: { 'Content-Type': 'application/json' },
        //   body: JSON.stringify({
        //     position: boardState,
        //     from: toAlgebraic(from),
        //     to: toAlgebraic(to),
        //     currentPlayer
        //   })
        // })
        // const data = await response.json()
        // if (!response.ok) throw new Error(data.message)

        // For demo purposes, we'll update the board state locally
        const newBoardState = JSON.parse(
          JSON.stringify(boardState),
        ) as BoardState;
        const piece = newBoardState[from.row][from.col];
        const capturedPiece = newBoardState[to.row][to.col];

        // Move the piece
        newBoardState[to.row][to.col] = piece;
        newBoardState[from.row][from.col] = null;

        // Add to move history
        const newMove: Move = {
          from: toAlgebraic(from),
          to: toAlgebraic(to),
          piece: piece || "",
          notation: `${piece}${toAlgebraic(from)}-${toAlgebraic(to)}`,
        };

        if (capturedPiece) {
          newMove.capture = capturedPiece;
          newMove.notation = `${piece}${toAlgebraic(from)}x${toAlgebraic(to)}`;
        }

        // Update state
        setBoardState(newBoardState);
        setMoveHistory([...moveHistory, newMove]);
        setCurrentPlayer(currentPlayer === "white" ? "black" : "white");
        setSelectedSquare(null);
        setPossibleMoves([]);

        // In a real app, the backend would return the new game status
        // For demo, we'll just keep it active
        // setGameStatus(data.gameStatus)
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to make move");
      } finally {
        setIsLoading(false);
      }
    },
    [boardState, currentPlayer, moveHistory],
  );

  // Reset the game
  const resetGame = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      // This would be replaced with an actual API call to your backend
      // Example API call:
      // const response = await fetch('/api/chess/new-game', {
      //   method: 'POST'
      // })
      // const data = await response.json()
      // if (!response.ok) throw new Error(data.message)

      // Reset the game state
      setBoardState(initialBoardState);
      setCurrentPlayer("white");
      setGameStatus("active");
      setSelectedSquare(null);
      setPossibleMoves([]);
      setMoveHistory([]);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to reset game");
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Undo the last move
  const undoMove = useCallback(async () => {
    try {
      if (moveHistory.length === 0) return;

      setIsLoading(true);
      setError(null);

      // This would be replaced with an actual API call to your backend
      // Example API call:
      // const response = await fetch('/api/chess/undo-move', {
      //   method: 'POST',
      //   headers: { 'Content-Type': 'application/json' },
      //   body: JSON.stringify({
      //     position: boardState,
      //     moveHistory
      //   })
      // })
      // const data = await response.json()
      // if (!response.ok) throw new Error(data.message)

      // For demo purposes, we'll just remove the last move
      // In a real app, the backend would handle this and return the new board state
      const newMoveHistory = [...moveHistory];
      newMoveHistory.pop();

      // This is a simplified implementation - in a real app, the backend would return the correct board state
      // Here we're just resetting to the initial state if there are no moves left
      if (newMoveHistory.length === 0) {
        setBoardState(initialBoardState);
        setCurrentPlayer("white");
      } else {
        // Toggle the current player
        setCurrentPlayer(currentPlayer === "white" ? "black" : "white");
      }

      setMoveHistory(newMoveHistory);
      setSelectedSquare(null);
      setPossibleMoves([]);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to undo move");
    } finally {
      setIsLoading(false);
    }
  }, [boardState, currentPlayer, moveHistory]);

  return {
    boardState,
    currentPlayer,
    gameStatus,
    moveHistory,
    selectedSquare,
    possibleMoves,
    handleSquareClick,
    makeMove,
    resetGame,
    undoMove,
    isLoading,
    error,
  };
}
