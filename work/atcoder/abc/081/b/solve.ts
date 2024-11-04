import * as fs from "fs";

const lines = fs.readFileSync("/dev/stdin", "utf8").split("\n");
const N = Number(lines[0]);
let A = lines[1].split(" ").map(Number);

let count = 0;

while (A.every((x) => x % 2 == 0)) {
  A = A.map((x) => x >> 1);
  count++;
}

console.log(count);
