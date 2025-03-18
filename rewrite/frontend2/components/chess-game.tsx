"use client";
import { Card, CardContent } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useToast } from "@/hooks/use-toast";
import ChessBoard from "@/components/chess-board";
import GameInfo from "@/components/game-info";
import MoveHistory from "@/components/move-history";
import GameControls from "@/components/game-controls";
import { useChessGame } from "@/hooks/use-chess-game";

export default function ChessGame() {
  const { toast } = useToast();
  const {
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
  } = useChessGame();

  // Show error toast if there's an error
  if (error) {
    toast({
      variant: "destructive",
      title: "Error",
      description: error,
    });
  }

  return (
    <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div className="lg:col-span-2">
        <Card>
          <CardContent className="p-6">
            <ChessBoard
              boardState={boardState}
              selectedSquare={selectedSquare}
              possibleMoves={possibleMoves}
              onSquareClick={handleSquareClick}
              currentPlayer={currentPlayer}
              isLoading={isLoading}
            />
          </CardContent>
        </Card>
      </div>
      <div className="space-y-6">
        <GameInfo currentPlayer={currentPlayer} gameStatus={gameStatus} />

        <Tabs defaultValue="moves" className="w-full">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="moves">Move History</TabsTrigger>
            <TabsTrigger value="controls">Game Controls</TabsTrigger>
          </TabsList>
          <TabsContent value="moves">
            <Card>
              <CardContent className="p-4">
                <MoveHistory moves={moveHistory} />
              </CardContent>
            </Card>
          </TabsContent>
          <TabsContent value="controls">
            <Card>
              <CardContent className="p-4">
                <GameControls
                  onReset={resetGame}
                  onUndo={undoMove}
                  gameStatus={gameStatus}
                  isLoading={isLoading}
                />
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
}
