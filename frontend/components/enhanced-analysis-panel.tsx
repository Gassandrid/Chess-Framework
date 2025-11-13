"use client";

import { useState, useEffect } from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { BarChart3, Cpu, RefreshCw, Loader2 } from "lucide-react";
import {
  analyzePosition,
  boardToFEN,
  formatEvaluation,
  getEvaluationColor,
  ComprehensiveAnalysis,
} from "@/lib/engine-api";

type PieceType =
  | "p" | "r" | "n" | "b" | "q" | "k"
  | "P" | "R" | "N" | "B" | "Q" | "K"
  | null;

interface EnhancedAnalysisPanelProps {
  boardState: PieceType[][];
  currentPlayer: "white" | "black";
}

export default function EnhancedAnalysisPanel({
  boardState,
  currentPlayer,
}: EnhancedAnalysisPanelProps) {
  const [analysis, setAnalysis] = useState<ComprehensiveAnalysis | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const runAnalysis = async () => {
    try {
      setIsAnalyzing(true);
      setError(null);

      const fen = boardToFEN(boardState, currentPlayer);
      const result = await analyzePosition(fen, 8);
      setAnalysis(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to analyze position");
    } finally {
      setIsAnalyzing(false);
    }
  };

  // Auto-analyze on board state change
  useEffect(() => {
    runAnalysis();
  }, [boardState, currentPlayer]);

  // Calculate progress bar value from evaluation
  const getProgressValue = (eval: number) => {
    // Clamp between -1000 and +1000, map to 0-100
    const clamped = Math.max(-1000, Math.min(1000, eval));
    return ((clamped + 1000) / 2000) * 100;
  };

  return (
    <div>
      <Tabs defaultValue="evaluation">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="evaluation">
            <BarChart3 className="h-4 w-4 mr-2" />
            Evaluation
          </TabsTrigger>
          <TabsTrigger value="engine">
            <Cpu className="h-4 w-4 mr-2" />
            Analysis
          </TabsTrigger>
        </TabsList>

        <TabsContent value="evaluation">
          <Card>
            <CardHeader>
              <div className="flex items-center justify-between">
                <CardTitle>Position Evaluation</CardTitle>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={runAnalysis}
                  disabled={isAnalyzing}
                >
                  {isAnalyzing ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <RefreshCw className="h-4 w-4" />
                  )}
                </Button>
              </div>
              {analysis && (
                <CardDescription>
                  {analysis.static_eval > 0 ? "White" : "Black"} advantage:{" "}
                  <span
                    className={`font-bold ${getEvaluationColor(analysis.static_eval)}`}
                  >
                    {formatEvaluation(analysis.static_eval)}
                  </span>
                </CardDescription>
              )}
              {error && <CardDescription className="text-red-500">{error}</CardDescription>}
            </CardHeader>
            <CardContent className="space-y-4">
              {isAnalyzing && !analysis && (
                <div className="flex items-center justify-center py-8">
                  <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
                </div>
              )}

              {analysis && (
                <>
                  <div>
                    <div className="flex justify-between mb-1">
                      <span className="text-sm font-medium">White</span>
                      <span
                        className={`text-sm font-medium ${getEvaluationColor(analysis.static_eval)}`}
                      >
                        {formatEvaluation(analysis.static_eval)}
                      </span>
                    </div>
                    <Progress
                      value={getProgressValue(analysis.static_eval)}
                      className="h-2"
                    />
                  </div>

                  {analysis.positional_features && (
                    <div className="space-y-2">
                      <h4 className="text-sm font-medium">Material Balance</h4>
                      <div className="grid grid-cols-2 gap-2">
                        <div className="flex items-center justify-between">
                          <span className="text-sm">Evaluation</span>
                          <span className="text-sm font-medium">
                            {formatEvaluation(analysis.positional_features.material_balance)}
                          </span>
                        </div>
                      </div>
                    </div>
                  )}

                  {analysis.tactical_features && (
                    <div className="space-y-2">
                      <h4 className="text-sm font-medium">Tactical Opportunities</h4>
                      <div className="grid grid-cols-2 gap-2 text-sm">
                        <div className="flex items-center justify-between">
                          <span>Checks available</span>
                          <Badge variant="secondary">
                            {analysis.tactical_features.checks_available}
                          </Badge>
                        </div>
                        <div className="flex items-center justify-between">
                          <span>Captures available</span>
                          <Badge variant="secondary">
                            {analysis.tactical_features.captures_available}
                          </Badge>
                        </div>
                      </div>
                    </div>
                  )}

                  {analysis.best_move && (
                    <div className="space-y-2">
                      <h4 className="text-sm font-medium">Best Move</h4>
                      <div className="flex items-center justify-between p-2 bg-muted rounded-md">
                        <span className="font-mono text-sm">{analysis.best_move}</span>
                        {analysis.search_eval !== null && (
                          <span
                            className={`text-sm font-medium ${getEvaluationColor(analysis.search_eval)}`}
                          >
                            {formatEvaluation(analysis.search_eval)}
                          </span>
                        )}
                      </div>
                    </div>
                  )}
                </>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="engine">
          <Card>
            <CardHeader>
              <CardTitle>Engine Analysis</CardTitle>
              {analysis && (
                <CardDescription>
                  Top moves for {currentPlayer}
                </CardDescription>
              )}
            </CardHeader>
            <CardContent className="space-y-4">
              {isAnalyzing && !analysis && (
                <div className="flex items-center justify-center py-8">
                  <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
                </div>
              )}

              {analysis && analysis.top_moves && analysis.top_moves.length > 0 && (
                <div className="space-y-3">
                  <h4 className="text-sm font-medium">Top Moves</h4>
                  {analysis.top_moves.map((move, index) => (
                    <div
                      key={index}
                      className="flex items-center justify-between p-2 bg-muted rounded-md hover:bg-muted/80 transition-colors"
                    >
                      <div className="flex items-center gap-2">
                        <span className="text-xs text-muted-foreground w-4">
                          {index + 1}.
                        </span>
                        <span className="font-mono text-sm">{move.mv}</span>
                      </div>
                      <span
                        className={`text-sm font-medium ${getEvaluationColor(move.score)}`}
                      >
                        {formatEvaluation(move.score)}
                      </span>
                    </div>
                  ))}
                </div>
              )}

              {analysis && analysis.evaluation_breakdown && (
                <div className="space-y-2">
                  <h4 className="text-sm font-medium">Evaluation Breakdown</h4>
                  <div className="space-y-1 text-sm">
                    <div className="flex items-center justify-between p-1">
                      <span>Classical</span>
                      <span className="font-mono">
                        {formatEvaluation(analysis.evaluation_breakdown.classical_score)}
                      </span>
                    </div>
                    {analysis.evaluation_breakdown.nn_score !== null && (
                      <div className="flex items-center justify-between p-1">
                        <span>Neural Network</span>
                        <span className="font-mono">
                          {formatEvaluation(analysis.evaluation_breakdown.nn_score)}
                        </span>
                      </div>
                    )}
                    {analysis.evaluation_breakdown.deep_nn_score !== null && (
                      <div className="flex items-center justify-between p-1">
                        <span>Deep NN</span>
                        <span className="font-mono">
                          {formatEvaluation(analysis.evaluation_breakdown.deep_nn_score)}
                        </span>
                      </div>
                    )}
                    <div className="flex items-center justify-between p-1 border-t mt-1 pt-2">
                      <span className="font-medium">Final Score</span>
                      <span
                        className={`font-mono font-medium ${getEvaluationColor(analysis.evaluation_breakdown.final_score)}`}
                      >
                        {formatEvaluation(analysis.evaluation_breakdown.final_score)}
                      </span>
                    </div>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
