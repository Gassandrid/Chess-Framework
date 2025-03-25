import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Search, Filter, Download } from "lucide-react";

export default function MoveHistoryPanel() {
  // Sample data for demonstration
  const games = [
    {
      id: 1,
      white: "Player 1",
      black: "Player 2",
      result: "1-0",
      date: "2023-05-15",
      moves: 42,
    },
    {
      id: 2,
      white: "Engine A",
      black: "Engine B",
      result: "0-1",
      date: "2023-05-14",
      moves: 36,
    },
    {
      id: 3,
      white: "Player 1",
      black: "Engine A",
      result: "½-½",
      date: "2023-05-13",
      moves: 60,
    },
    {
      id: 4,
      white: "Engine B",
      black: "Player 2",
      result: "1-0",
      date: "2023-05-12",
      moves: 28,
    },
    {
      id: 5,
      white: "Player 3",
      black: "Player 4",
      result: "0-1",
      date: "2023-05-11",
      moves: 45,
    },
  ];

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Game History</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center space-x-2 mb-4">
            <div className="relative flex-1">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                type="search"
                placeholder="Search games..."
                className="pl-8"
              />
            </div>
            <Button variant="outline" size="icon">
              <Filter className="h-4 w-4" />
              <span className="sr-only">Filter</span>
            </Button>
            <Button variant="outline" size="icon">
              <Download className="h-4 w-4" />
              <span className="sr-only">Export</span>
            </Button>
          </div>

          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>White</TableHead>
                  <TableHead>Black</TableHead>
                  <TableHead>Result</TableHead>
                  <TableHead>Date</TableHead>
                  <TableHead className="text-right">Moves</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {games.map((game) => (
                  <TableRow
                    key={game.id}
                    className="cursor-pointer hover:bg-muted/50"
                  >
                    <TableCell>{game.white}</TableCell>
                    <TableCell>{game.black}</TableCell>
                    <TableCell>{game.result}</TableCell>
                    <TableCell>{game.date}</TableCell>
                    <TableCell className="text-right">{game.moves}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Game Details</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-4 mb-4">
            <div>
              <p className="text-sm font-medium text-muted-foreground">White</p>
              <p>Player 1</p>
            </div>
            <div>
              <p className="text-sm font-medium text-muted-foreground">Black</p>
              <p>Player 2</p>
            </div>
            <div>
              <p className="text-sm font-medium text-muted-foreground">
                Result
              </p>
              <p>1-0</p>
            </div>
            <div>
              <p className="text-sm font-medium text-muted-foreground">Date</p>
              <p>May 15, 2023</p>
            </div>
          </div>

          <div className="h-[300px]">
            <ScrollArea className="h-full">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead className="w-[60px]">#</TableHead>
                    <TableHead>White</TableHead>
                    <TableHead>Black</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  <TableRow>
                    <TableCell>1</TableCell>
                    <TableCell>e4</TableCell>
                    <TableCell>e5</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>2</TableCell>
                    <TableCell>Nf3</TableCell>
                    <TableCell>Nc6</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>3</TableCell>
                    <TableCell>Bb5</TableCell>
                    <TableCell>a6</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>4</TableCell>
                    <TableCell>Ba4</TableCell>
                    <TableCell>Nf6</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>5</TableCell>
                    <TableCell>O-O</TableCell>
                    <TableCell>Be7</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>6</TableCell>
                    <TableCell>Re1</TableCell>
                    <TableCell>b5</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>7</TableCell>
                    <TableCell>Bb3</TableCell>
                    <TableCell>d6</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>8</TableCell>
                    <TableCell>c3</TableCell>
                    <TableCell>O-O</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </ScrollArea>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
