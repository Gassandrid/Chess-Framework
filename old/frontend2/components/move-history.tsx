import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

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

interface MoveHistoryProps {
  moves: Move[];
}

export default function MoveHistory({ moves }: MoveHistoryProps) {
  // Group moves into pairs for white and black
  const moveRows = [];
  for (let i = 0; i < moves.length; i += 2) {
    moveRows.push({
      number: Math.floor(i / 2) + 1,
      white: moves[i],
      black: moves[i + 1] || null,
    });
  }

  return (
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
            {moveRows.length > 0 ? (
              moveRows.map((row) => (
                <TableRow key={row.number}>
                  <TableCell className="font-medium">{row.number}</TableCell>
                  <TableCell>{row.white?.notation}</TableCell>
                  <TableCell>{row.black?.notation}</TableCell>
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={3}
                  className="text-center py-4 text-muted-foreground"
                >
                  No moves yet
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  );
}
