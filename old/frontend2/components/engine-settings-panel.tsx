import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
  CardFooter,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { Slider } from "@/components/ui/slider";
import { Play, Plus, Trash2 } from "lucide-react";

export default function EngineSettingsPanel() {
  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Engine Match</CardTitle>
          <CardDescription>
            Configure engines to play against each other
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base">Engine 1 (White)</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="engine1">Select Engine</Label>
                  <Select defaultValue="stockfish">
                    <SelectTrigger>
                      <SelectValue placeholder="Select engine" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="stockfish">Stockfish 16</SelectItem>
                      <SelectItem value="lc0">Leela Chess Zero</SelectItem>
                      <SelectItem value="komodo">Komodo 14</SelectItem>
                    </SelectContent>
                  </Select>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="depth1">Search Depth</Label>
                  <div className="flex items-center space-x-2">
                    <Slider defaultValue={[20]} max={30} step={1} />
                    <span className="w-12 text-center">20</span>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="time1">Time per Move (sec)</Label>
                  <Input type="number" id="time1" defaultValue="5" />
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base">Engine 2 (Black)</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="engine2">Select Engine</Label>
                  <Select defaultValue="lc0">
                    <SelectTrigger>
                      <SelectValue placeholder="Select engine" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="stockfish">Stockfish 16</SelectItem>
                      <SelectItem value="lc0">Leela Chess Zero</SelectItem>
                      <SelectItem value="komodo">Komodo 14</SelectItem>
                    </SelectContent>
                  </Select>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="depth2">Search Depth</Label>
                  <div className="flex items-center space-x-2">
                    <Slider defaultValue={[18]} max={30} step={1} />
                    <span className="w-12 text-center">18</span>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="time2">Time per Move (sec)</Label>
                  <Input type="number" id="time2" defaultValue="5" />
                </div>
              </CardContent>
            </Card>
          </div>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Match Settings</Label>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="games">Number of Games</Label>
                  <Input type="number" id="games" defaultValue="10" />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="opening">Opening Book</Label>
                  <Select defaultValue="standard">
                    <SelectTrigger>
                      <SelectValue placeholder="Select opening book" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="standard">Standard</SelectItem>
                      <SelectItem value="masters">Masters</SelectItem>
                      <SelectItem value="varied">Varied</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>

            <div className="flex items-center space-x-2">
              <Switch id="swap" />
              <Label htmlFor="swap">Swap colors after each game</Label>
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <Button className="w-full">
            <Play className="mr-2 h-4 w-4" />
            Start Engine Match
          </Button>
        </CardFooter>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Manage Engines</CardTitle>
          <CardDescription>
            Add, remove, or configure chess engines
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="rounded-md border">
              <div className="flex items-center justify-between p-4">
                <div className="flex items-center space-x-4">
                  <div>
                    <p className="font-medium">Stockfish 16</p>
                    <p className="text-sm text-muted-foreground">
                      Version: 16.0
                    </p>
                  </div>
                </div>
                <Button variant="outline" size="sm">
                  Configure
                </Button>
              </div>
              <div className="border-t" />
              <div className="flex items-center justify-between p-4">
                <div className="flex items-center space-x-4">
                  <div>
                    <p className="font-medium">Leela Chess Zero</p>
                    <p className="text-sm text-muted-foreground">
                      Version: 0.29.0
                    </p>
                  </div>
                </div>
                <Button variant="outline" size="sm">
                  Configure
                </Button>
              </div>
              <div className="border-t" />
              <div className="flex items-center justify-between p-4">
                <div className="flex items-center space-x-4">
                  <div>
                    <p className="font-medium">Komodo 14</p>
                    <p className="text-sm text-muted-foreground">
                      Version: 14.1
                    </p>
                  </div>
                </div>
                <Button variant="outline" size="sm">
                  Configure
                </Button>
              </div>
            </div>

            <div className="flex justify-between">
              <Button variant="outline" size="sm">
                <Plus className="mr-2 h-4 w-4" />
                Add Engine
              </Button>
              <Button variant="outline" size="sm" className="text-destructive">
                <Trash2 className="mr-2 h-4 w-4" />
                Remove Engine
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
