#!/usr/bin/env node

// something is wrong with the space
// based on https://gist.github.com/jasonherald/9901990
// make input with , in node work!

let ptr = 0;
const ins = Array();
const tape = 30000;
const loop = Array();

let ctr = 0;

for (let i = 0; i < tape; i++) {
  ins[i] = 0;
}

function bf(msg) {
  x = msg.split("");
  for (ctr = 0; ctr < x.length; ctr++) {
    switch (x[ctr]) {
      case ">":
        ptr++;
        break;
      case "<":
        ptr--;
        break;
      case "+":
        ins[ptr]++;
        break;
      case "-":
        ins[ptr]--;
        break;
      case ".":
        process.stdout.write(String.fromCharCode(ins[ptr]));
        // console.log(String.fromCharCode(ins[ptr]));
        break;
      case ",":
        ins[ptr] = prompt("Input");
        break;
      case "[":
        if (ins[ptr] !== 0) loop.push({ ptr: ptr, ctr: ctr });
        break;
      case "]":
        if (ins[ptr] === 0) loop.pop();
        else ctr = loop[loop.length - 1].ctr;
        break;
    }
  }
}

const readline = require("readline").createInterface({
  input: process.stdin,
  output: process.stdout,
});

readline.question("write your bf code here: \n", (msg) => {
  console.log(bf(msg));
  readline.close();
});
