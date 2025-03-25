import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

type Player = "white" | "black";
type GameStatus = "active" | "check" | "checkmate" | "stalemate" | "draw";

interface GameInfoProps {
  currentPlayer: Player;
  gameStatus: GameStatus;
}

export default function GameInfo({ currentPlayer, gameStatus }: GameInfoProps) {
  const statusMessages = {
    active: "Game in progress",
    check: `${currentPlayer === "white" ? "White" : "Black"} is in check!`,
    checkmate: `Checkmate! ${currentPlayer === "white" ? "Black" : "White"} wins!`,
    stalemate: "Stalemate! The game is a draw.",
    draw: "Draw! The game is over.",
  };

  const statusVariants: Record<
    GameStatus,
    "default" | "secondary" | "destructive" | "outline"
  > = {
    active: "default",
    check: "secondary",
    checkmate: "destructive",
    stalemate: "outline",
    draw: "outline",
  };

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle>Game Status</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex justify-between items-center">
          <span className="font-medium">Current Turn:</span>
          <Badge
            variant={currentPlayer === "white" ? "default" : "outline"}
            className="capitalize"
          >
            {currentPlayer}
          </Badge>
        </div>

        <div className="flex justify-between items-center">
          <span className="font-medium">Status:</span>
          <Badge variant={statusVariants[gameStatus]}>
            {statusMessages[gameStatus]}
          </Badge>
        </div>
      </CardContent>
    </Card>
  );
}
