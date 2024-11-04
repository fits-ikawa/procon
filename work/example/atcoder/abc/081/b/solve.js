var lines = require("fs").readFileSync("/dev/stdin", "utf8").split("\n");
var N = Number(lines[0]);
var A = lines[1].split(" ").map(Number);

count = 0;

while (A.every((x) => x % 2 == 0)) {
  A = A.map((x) => x >> 1);
  count++;
}

console.log(count);
