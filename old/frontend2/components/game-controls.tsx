"use client";

import { Button } from "@/components/ui/button";
import { RotateCcw, RefreshCw, Save, Upload, Play } from "lucide-react";

type GameStatus = "active" | "check" | "checkmate" | "stalemate" | "draw";

interface GameControlsProps {
  onReset: () => void;
  onUndo: () => void;
  gameStatus: GameStatus;
  isLoading: boolean;
}

export default function GameControls({
  onReset,
  onUndo,
  gameStatus,
  isLoading,
}: GameControlsProps) {
  const isGameOver = ["checkmate", "stalemate", "draw"].includes(gameStatus);

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-2 gap-2">
        <Button onClick={onUndo} variant="outline" disabled={isLoading}>
          <RotateCcw className="mr-2 h-4 w-4" />
          Undo Move
        </Button>

        <Button
          onClick={onReset}
          variant={isGameOver ? "default" : "outline"}
          disabled={isLoading}
        >
          <RefreshCw className="mr-2 h-4 w-4" />
          New Game
        </Button>
      </div>

      <div className="grid grid-cols-2 gap-2">
        <Button variant="secondary" disabled={isLoading}>
          <Save className="mr-2 h-4 w-4" />
          Save Game
        </Button>

        <Button variant="secondary" disabled={isLoading}>
          <Upload className="mr-2 h-4 w-4" />
          Load Game
        </Button>
      </div>

      <Button className="w-full" variant="default" disabled={isLoading}>
        <Play className="mr-2 h-4 w-4" />
        Auto Play (Engine vs Engine)
      </Button>
    </div>
  );
}
