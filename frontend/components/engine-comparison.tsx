"use client";

import { useState, useEffect } from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { Play, Loader2 } from "lucide-react";
import { listEngines, getBestMove, EngineInfo } from "@/lib/engine-api";

const samplePosition = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; // Starting position

export default function EngineComparison() {
  const [engines, setEngines] = useState<EngineInfo[]>([]);
  const [engine1, setEngine1] = useState<string>("v1");
  const [engine2, setEngine2] = useState<string>("v3");
  const [isComparing, setIsComparing] = useState(false);
  const [results, setResults] = useState<{
    engine1: { move: string; nodes: number; time: number; nps: number } | null;
    engine2: { move: string; nodes: number; time: number; nps: number } | null;
  }>({ engine1: null, engine2: null });
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadEngines();
  }, []);

  const loadEngines = async () => {
    try {
      const engineList = await listEngines();
      setEngines(engineList);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load engines");
    }
  };

  const runComparison = async () => {
    try {
      setIsComparing(true);
      setError(null);
      setResults({ engine1: null, engine2: null });

      // Run both engines in parallel
      const [result1, result2] = await Promise.all([
        getBestMove(samplePosition, engine1, 6, 5000),
        getBestMove(samplePosition, engine2, 6, 5000),
      ]);

      setResults({
        engine1: {
          move: result1.best_move,
          nodes: result1.nodes,
          time: result1.time_ms,
          nps: result1.nps,
        },
        engine2: {
          move: result2.best_move,
          nodes: result2.nodes,
          time: result2.time_ms,
          nps: result2.nps,
        },
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to compare engines");
    } finally {
      setIsComparing(false);
    }
  };

  const formatNumber = (num: number) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num.toString();
  };

  const getEngineInfo = (id: string) => {
    return engines.find((e) => e.id === id);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Engine Comparison</CardTitle>
        <CardDescription>
          Compare different engine versions side by side
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {error && (
          <div className="p-3 bg-destructive/10 text-destructive rounded-md text-sm">
            {error}
          </div>
        )}

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Engine 1</label>
            <Select value={engine1} onValueChange={setEngine1}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {engines.map((engine) => (
                  <SelectItem key={engine.id} value={engine.id}>
                    {engine.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {getEngineInfo(engine1) && (
              <p className="text-xs text-muted-foreground">
                {getEngineInfo(engine1)!.description}
              </p>
            )}
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Engine 2</label>
            <Select value={engine2} onValueChange={setEngine2}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {engines.map((engine) => (
                  <SelectItem key={engine.id} value={engine.id}>
                    {engine.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {getEngineInfo(engine2) && (
              <p className="text-xs text-muted-foreground">
                {getEngineInfo(engine2)!.description}
              </p>
            )}
          </div>
        </div>

        <Button
          onClick={runComparison}
          disabled={isComparing || engine1 === engine2}
          className="w-full"
        >
          {isComparing ? (
            <>
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              Comparing...
            </>
          ) : (
            <>
              <Play className="mr-2 h-4 w-4" />
              Run Comparison
            </>
          )}
        </Button>

        {(results.engine1 || results.engine2) && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-3">
              <h3 className="font-medium text-sm flex items-center gap-2">
                {getEngineInfo(engine1)?.name}
                {results.engine1 && results.engine2 && results.engine1.nps > results.engine2.nps && (
                  <Badge variant="default" className="text-xs">Faster</Badge>
                )}
              </h3>
              {results.engine1 ? (
                <div className="space-y-2 p-3 bg-muted rounded-md">
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Best Move</span>
                    <span className="font-mono font-medium">{results.engine1.move}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Nodes</span>
                    <span className="font-mono">{formatNumber(results.engine1.nodes)}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Time</span>
                    <span className="font-mono">{results.engine1.time}ms</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">NPS</span>
                    <span className="font-mono">{formatNumber(results.engine1.nps)}</span>
                  </div>
                </div>
              ) : isComparing ? (
                <div className="flex items-center justify-center py-8">
                  <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
                </div>
              ) : null}
            </div>

            <div className="space-y-3">
              <h3 className="font-medium text-sm flex items-center gap-2">
                {getEngineInfo(engine2)?.name}
                {results.engine1 && results.engine2 && results.engine2.nps > results.engine1.nps && (
                  <Badge variant="default" className="text-xs">Faster</Badge>
                )}
              </h3>
              {results.engine2 ? (
                <div className="space-y-2 p-3 bg-muted rounded-md">
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Best Move</span>
                    <span className="font-mono font-medium">{results.engine2.move}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Nodes</span>
                    <span className="font-mono">{formatNumber(results.engine2.nodes)}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Time</span>
                    <span className="font-mono">{results.engine2.time}ms</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">NPS</span>
                    <span className="font-mono">{formatNumber(results.engine2.nps)}</span>
                  </div>
                </div>
              ) : isComparing ? (
                <div className="flex items-center justify-center py-8">
                  <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
                </div>
              ) : null}
            </div>
          </div>
        )}

        {results.engine1 && results.engine2 && results.engine1.move !== results.engine2.move && (
          <div className="p-3 bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800 rounded-md">
            <p className="text-sm text-yellow-800 dark:text-yellow-200">
              <strong>Note:</strong> The engines chose different best moves! This demonstrates
              how different search techniques and evaluations can lead to different strategic
              decisions.
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
