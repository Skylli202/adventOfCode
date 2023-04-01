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

const warehouseWithCrane9000 = new Warehouse();
const warehouseWithCrane9001 = new Warehouse();
warehouseWithCrane9000.deserialize(serializedWarehouse);
warehouseWithCrane9001.deserialize(serializedWarehouse);
console.log('Warehouse before crane orders executed:');
console.log(warehouseWithCrane9000.toString());

let craneOrders = serializedCraneOrders
  .split(/\n|\r\n/)
  .map((serializedCraneOrder) => {
    return parseCraneOrder(serializedCraneOrder);
  });

for (let i = 0; i < craneOrders.length; i++) {
  console.log(craneOrders[i]);
  warehouseWithCrane9000.moveCrates(...craneOrders[i]);
  warehouseWithCrane9001.moveCratesConservatively(...craneOrders[i]);
}

console.log('Warehouse after crane orders executed:');
console.log(warehouseWithCrane9000.toString());
console.log(
  'Puzzle solution: ' + warehouseWithCrane9000.getCratesOnTopOfStacks().join('')
);
console.log(
  'Puzzle solution: ' + warehouseWithCrane9001.getCratesOnTopOfStacks().join('')
);
