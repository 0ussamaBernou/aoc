import { BunFile } from "bun";
import path from "path";

// read file
const path: string = "day1.input";

const file: BunFile = Bun.file(path);

const contents: string = await file.text();

// console.log(contents);

// split content by blankline

const blocks = contents.split("\n\n");

// map block index to total

const calories = blocks.map((block) => {
  return block.split("\n");
});

const totals: number[] = calories.map((block) => {
  return block.map((calorie) => {
    return +calorie; // or parseInt(calorie)
  })
    .filter((num) => !isNaN(num))
    .reduce((acc, current) => acc + current, 0);
});

// console.log(totals);
// return the max total
// const max = Math.max(...totals);
const sortedTotals = totals.sort((a, b) => b - a);
const max = sortedTotals[0];

console.log(max);

const sum3 = sortedTotals.slice(0, 3).reduce((a, b) => a + b, 0);
console.log(`part two: ${sum3}`);
