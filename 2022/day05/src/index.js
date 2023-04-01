import { readFileSync } from 'fs';
import { Warehouse } from './Warehouse.js';
import { parseCraneOrder } from './parser.js';

// const puzzleSample = readFileSync('./assets/puzzle-sample.txt', 'utf-8');
const puzzleInput = readFileSync('./assets/puzzle-input.txt', 'utf-8');

const [inputSerializedWarehouse, serializedCraneOrders] =
  puzzleInput.split(/\r\n\r\n|\n\n/);

// Explode the input per line
let serializedWarehouse = inputSerializedWarehouse.split(/\r\n|\n/);
// pop the last line that we do not need
serializedWarehouse.pop();
// join the array into a string that has the shape of a serialized warehouse
serializedWarehouse = serializedWarehouse.join('\n');

const warehouse = new Warehouse();
warehouse.deserialize(serializedWarehouse);
console.log('Warehouse before crane orders executed:');
console.log(warehouse.toString());
// console.log('---');
// console.log(serializedCraneOrders);
// console.log('');
// console.log('');

let craneOrders = serializedCraneOrders
  .split(/\n|\r\n/)
  .map((serializedCraneOrder) => {
    return parseCraneOrder(serializedCraneOrder);
  });

for (let i = 0; i < craneOrders.length; i++) {
  console.log(craneOrders[i]);
  warehouse.moveCrates(...craneOrders[i]);
}

console.log('Warehouse after crane orders executed:');
console.log(warehouse.toString());
console.log('Puzzle solution: ' + warehouse.getCratesOnTopOfStacks().join(''));
