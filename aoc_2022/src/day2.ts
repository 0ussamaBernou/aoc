import { BunFile } from "bun";

enum Move {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

namespace Move {
  export function get(move: string): Move {
    switch (move) {
      case "A":
        return Move.Rock;
      case "B":
        return Move.Paper;
      case "C":
        return Move.Scissors;
      default:
        throw new Error("Unexpected move");
    }
  }
}

async function part_2(filename: string): Promise<number> {
  // read file
  const file: BunFile = Bun.file(filename);
  const contents: string = await file.text();
  return contents.trim().split("\n").map((line) => {
    const moves: string[] = line.split(" ");
    let opMove = Move.get(moves[0]);
    let res: number = 0;
    switch (moves[1]) {
      case "Y":
        return 3 + opMove;
      case "X":
        switch (opMove) {
          case Move.Scissors:
            return 0 + Move.Paper;
          case Move.Paper:
            return 0 + Move.Rock;
          case Move.Rock:
            return 0 + Move.Scissors;
        }
      case "Z":
        switch (opMove) {
          case Move.Scissors:
            return 6 + Move.Rock;
          case Move.Paper:
            return 6 + Move.Scissors;
          case Move.Rock:
            return 6 + Move.Paper;
        }

      default:
        break;
    }
    return res;
  }).reduce((acc, current) => acc + current, 0);
}

const filePath: string = "day2.input";
console.log(` part2 result ${await part_2(filePath)}`);
