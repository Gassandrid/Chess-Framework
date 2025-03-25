"use client";

import { useState, useCallback } from "react";

// simple piece type, for generating boards quickly
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

// initial board state
// TODO: I might make a load from FEN function for the
// frontend, would be needed for communication with
// the backend anyway
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

// converting from computer to human readable coordss
const toAlgebraic = (square: Square): string => {
  const file = String.fromCharCode(97 + square.col);
  const rank = 8 - square.row;
  return `${file}${rank}`;
};

// react hook for the chess game info and data
export function useChessGame() {
  const [boardState, setBoardState] = useState<BoardState>(initialBoardState);
  const [currentPlayer, setCurrentPlayer] = useState<Player>("white");
  const [gameStatus, setGameStatus] = useState<GameStatus>("active");
  const [selectedSquare, setSelectedSquare] = useState<Square | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<Square[]>([]);
  const [moveHistory, setMoveHistory] = useState<Move[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // WARNING: THIS IS MOST IMPORTANT IMPLEMENTATION TODO
  const fetchLegalMoves = useCallback(
    async (square: Square) => {
      try {
        setIsLoading(true);
        setError(null);

        // Convert board state to a format the Rust backend can understand
        // You might need to adjust this based on your backend's expected format
        // const response = await fetch(
        //   "http://localhost:3001/api/chess/legal-moves",
        //   {
        //     method: "POST",
        //     headers: { "Content-Type": "application/json" },
        //     body: JSON.stringify({
        //       position: boardState,
        //       square: toAlgebraic(square),
        //       currentPlayer,
        //     }),
        //   },
        // );
        //
        // if (!response.ok) {
        //   const errorData = await response.json();
        //   throw new Error(errorData.message || "Failed to fetch legal moves");
        // }
        //
        // const data = await response.json();

        // Convert the moves from algebraic notation to our Square format
        // Assuming the backend returns an array of algebraic notation moves like ["e4", "e5"]
        // return data.moves.map((move: string) => ({
        //   row: 8 - parseInt(move[1]),
        //   col: move.charCodeAt(0) - 97,
        // }));

        // Fallback to the mock implementation if the API call fails
        const piece = boardState[square.row][square.col];
        const mockMoves: Square[] = [];

        if (piece === "P" && square.row > 0) {
          mockMoves.push({ row: square.row - 1, col: square.col });
          if (square.row === 6)
            mockMoves.push({ row: square.row - 2, col: square.col });
        } else if (piece === "p" && square.row < 7) {
          mockMoves.push({ row: square.row + 1, col: square.col });
          if (square.row === 1)
            mockMoves.push({ row: square.row + 2, col: square.col });
        }

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

  // for the square clicking bheavior
  // know when to activate the legal moves
  const handleSquareClick = useCallback(
    async (square: Square) => {
      const piece = boardState[square.row][square.col];

      // fallback for if no piece is selected
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

      // given square already selected:

      // if clicking same square, deselect it
      if (
        selectedSquare.row === square.row &&
        selectedSquare.col === square.col
      ) {
        setSelectedSquare(null);
        setPossibleMoves([]);
        return;
      }

      // check if move second click is a valid move
      const isValidMove = possibleMoves.some(
        (move) => move.row === square.row && move.col === square.col,
      );

      if (isValidMove) {
        // mae the move
        makeMove(selectedSquare, square);
      } else {
        // if they press one of their own pieces again just select that piece
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
          // if empty or no valid
          setSelectedSquare(null);
          setPossibleMoves([]);
        }
      }
    },
    [boardState, currentPlayer, selectedSquare, possibleMoves, fetchLegalMoves],
  );

  // move bmaker callback
  const makeMove = useCallback(
    async (from: Square, to: Square) => {
      try {
        setIsLoading(true);
        setError(null);

        // Call the Rust backend to make the move
        const response = await fetch(
          "http://localhost:3001/api/chess/make-move",
          {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              position: boardState,
              from: toAlgebraic(from),
              to: toAlgebraic(to),
              currentPlayer,
            }),
          },
        );

        if (!response.ok) {
          const errorData = await response.json();
          throw new Error(errorData.message || "Failed to make move");
        }

        const data = await response.json();

        // Use the new board state returned from the backend
        // Assuming the backend returns a new board state and game information
        const newBoardState = data.boardState;
        const gameStatus = data.gameStatus;

        // Create the move history entry using data from backend
        const newMove: Move = {
          from: toAlgebraic(from),
          to: toAlgebraic(to),
          piece: data.piece || boardState[from.row][from.col] || "",
          capture: data.capture,
          promotion: data.promotion,
          check: data.check,
          checkmate: data.checkmate,
          notation:
            data.notation ||
            `${data.piece}${toAlgebraic(from)}-${toAlgebraic(to)}`,
        };

        // Update states based on backend response
        setBoardState(newBoardState);
        setMoveHistory([...moveHistory, newMove]);
        setCurrentPlayer(
          data.nextPlayer || (currentPlayer === "white" ? "black" : "white"),
        );
        setSelectedSquare(null);
        setPossibleMoves([]);
        setGameStatus(gameStatus);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to make move");
      } finally {
        setIsLoading(false);
      }
    },
    [boardState, currentPlayer, moveHistory],
  );

  // game reseter
  const resetGame = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      // Call the Rust backend to reset the game
      const response = await fetch("http://localhost:3001/api/chess/new-game", {
        method: "POST",
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || "Failed to reset game");
      }

      const data = await response.json();

      // Use the initial board state from backend or fallback to our default
      setBoardState(data.boardState || initialBoardState);
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

  // move undoer, i dont reallt know if i want this but
  // it was an easy implementation at the time,
  // might not be so easy on backend, esp with an engine
  const undoMove = useCallback(async () => {
    try {
      if (moveHistory.length === 0) return;

      setIsLoading(true);
      setError(null);

      // Call the Rust backend to undo the move
      const response = await fetch(
        "http://localhost:3001/api/chess/undo-move",
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            position: boardState,
            moveHistory,
          }),
        },
      );

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || "Failed to undo move");
      }

      const data = await response.json();

      // Use the previous board state from the backend
      setBoardState(data.boardState);
      setCurrentPlayer(data.currentPlayer);
      setGameStatus(data.gameStatus || "active");

      // Update move history by removing the last move
      const newMoveHistory = [...moveHistory];
      newMoveHistory.pop();
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
