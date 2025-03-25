import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Progress } from "@/components/ui/progress";
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  BarChart3,
  Cpu,
} from "lucide-react";
import ChessBoard from "@/components/chess-board";

// NOTE: This panel is not going to be implemented in
// this project on the backend, i am good with frontend
// but rust backend takes time! will complete in module 4

// Type definition to ensure we're using valid piece types
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

// a sample board game, connect this to DB later
const sampleBoardState: PieceType[][] = [
  ["r", null, "b", "q", "k", "b", null, "r"],
  ["p", "p", "p", null, null, "p", "p", "p"],
  [null, null, "n", "p", null, "n", null, null],
  [null, null, null, null, "p", null, null, null],
  [null, "P", null, null, "P", null, null, null],
  [null, null, "N", null, null, "N", null, null],
  ["P", null, "P", "P", null, "P", "P", "P"],
  ["R", null, "B", "Q", "K", "B", null, "R"],
];

// this is an analysis panel, it will be for users to look back on their games
// once connected to database, user should be able to acdess their games, and maybe we can
// generate some graphs from it
export default function AnalysisPanel() {
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2">
          <Card>
            <CardContent className="p-6">
              <ChessBoard
                boardState={sampleBoardState}
                selectedSquare={null}
                possibleMoves={[]}
                onSquareClick={() => {}}
                currentPlayer="white"
                isLoading={false}
              />
              <div className="flex items-center justify-center mt-4 space-x-2">
                <Button variant="outline" size="icon">
                  <SkipBack className="h-4 w-4" />
                </Button>
                <Button variant="outline" size="icon">
                  <Pause className="h-4 w-4" />
                </Button>
                <Button variant="outline" size="icon">
                  <Play className="h-4 w-4" />
                </Button>
                <Button variant="outline" size="icon">
                  <SkipForward className="h-4 w-4" />
                </Button>
              </div>
              <div className="mt-4">
                <Slider defaultValue={[15]} max={40} step={1} />
                <div className="flex justify-between text-xs text-muted-foreground mt-1">
                  {/* this is just a placeholder for now, will eventually implement */}
                  <span>Move 1</span>
                  <span>Current: Move 15</span>
                  <span>Move 40</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
        {/* this is the tab bar for choosing engines and whatnot */}
        <div>
          <Tabs defaultValue="evaluation">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="evaluation">
                <BarChart3 className="h-4 w-4 mr-2" />
                Evaluation
              </TabsTrigger>
              <TabsTrigger value="engine">
                <Cpu className="h-4 w-4 mr-2" />
                Engine
              </TabsTrigger>
            </TabsList>
            <TabsContent value="evaluation">
              <Card>
                <CardHeader>
                  <CardTitle>Position Evaluation</CardTitle>
                  <CardDescription>
                    Current advantage: +1.3 (White)
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div>
                    <div className="flex justify-between mb-1">
                      <span className="text-sm font-medium">White</span>
                      <span className="text-sm font-medium">+1.3</span>
                    </div>
                    <Progress value={63} className="h-2" />
                  </div>

                  <div className="space-y-2">
                    <h4 className="text-sm font-medium">Material</h4>
                    <div className="grid grid-cols-2 gap-2">
                      <div className="flex items-center justify-between">
                        <span className="text-sm">White</span>
                        <span className="text-sm font-medium">28</span>
                      </div>
                      <div className="flex items-center justify-between">
                        <span className="text-sm">Black</span>
                        <span className="text-sm font-medium">27</span>
                      </div>
                    </div>
                  </div>

                  <div className="space-y-2">
                    <h4 className="text-sm font-medium">Best Moves</h4>
                    <div className="space-y-1">
                      <div className="flex items-center justify-between">
                        <span className="text-sm">1. Nf3-d4</span>
                        <span className="text-sm font-medium">+1.8</span>
                      </div>
                      <div className="flex items-center justify-between">
                        <span className="text-sm">2. Qd1-c2</span>
                        <span className="text-sm font-medium">+1.5</span>
                      </div>
                      <div className="flex items-center justify-between">
                        <span className="text-sm">3. Bc1-e3</span>
                        <span className="text-sm font-medium">+1.2</span>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="engine">
              <Card>
                <CardHeader>
                  <CardTitle>Engine Analysis</CardTitle>
                  <CardDescription>Depth: 20 | Nodes: 15.4M</CardDescription>
                </CardHeader>
                {/* I had idea to implement natural language tooling to 
                explain why a move is good or bad but i dont know if 
                this is feasable */}
                <CardContent className="space-y-4">
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <span className="font-medium">Nf3-d4</span>
                      <span className="font-medium text-green-600">+1.8</span>
                    </div>
                    <p className="text-sm text-muted-foreground">
                      Knight to d4 controls the center and threatens the e6
                      pawn.
                    </p>
                  </div>

                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <span className="font-medium">Qd1-c2</span>
                      <span className="font-medium text-green-600">+1.5</span>
                    </div>
                    <p className="text-sm text-muted-foreground">
                      Queen to c2 prepares for kingside attack and supports the
                      center.
                    </p>
                  </div>

                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <span className="font-medium">Bc1-e3</span>
                      <span className="font-medium text-green-600">+1.2</span>
                    </div>
                    <p className="text-sm text-muted-foreground">
                      Bishop to e3 develops the piece and controls the d4
                      square.
                    </p>
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  );
}
