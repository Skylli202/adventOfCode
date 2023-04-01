import { describe, expect, it } from 'vitest';
import { Warehouse } from './Warehouse.js';

describe('Warehouse', () => {
  it('should be able to add a crate onto a specific stack', () => {
    const warehouse = new Warehouse();
    warehouse.addCrate(1, 'Z');
    expect(warehouse.getStack(1)).toEqual(['Z']);
    warehouse.addCrate(1, 'N');
    expect(warehouse.getStack(1)).toEqual(['Z', 'N']);

    warehouse.addCrate(2, 'M');
    expect(warehouse.getStack(2)).toEqual(['M']);
    warehouse.addCrate(2, 'C');
    expect(warehouse.getStack(2)).toEqual(['M', 'C']);
    warehouse.addCrate(2, 'D');
    expect(warehouse.getStack(2)).toEqual(['M', 'C', 'D']);

    warehouse.addCrate(3, 'P');
    expect(warehouse.getStack(3)).toEqual(['P']);
  });

  it('should be able to add a crate onto a each stack of the Warehouse', () => {
    const warehouse = new Warehouse();

    warehouse.addLayer(['Z', 'M', 'P']);
    expect(warehouse.getStack(1)).toEqual(['Z']);
    expect(warehouse.getStack(2)).toEqual(['M']);
    expect(warehouse.getStack(3)).toEqual(['P']);
    expect(warehouse.getLayer(1)).toEqual(['Z', 'M', 'P']);

    warehouse.addLayer(['N', 'C']);
    expect(warehouse.getStack(1)).toEqual(['Z', 'N']);
    expect(warehouse.getStack(2)).toEqual(['M', 'C']);
    expect(warehouse.getStack(3)).toEqual(['P']);
    expect(warehouse.getLayer(2)).toEqual(['N', 'C', '']);

    warehouse.addLayer(['', 'D']);
    expect(warehouse.getStack(1)).toEqual(['Z', 'N']);
    expect(warehouse.getStack(2)).toEqual(['M', 'C', 'D']);
    expect(warehouse.getStack(3)).toEqual(['P']);
    expect(warehouse.getLayer(3)).toEqual(['', 'D', '']);
  });

  it.skip('addLayer should ignore emptry string and white space when creating a layer', () => {
    const warehouse = new Warehouse();

    warehouse.addLayer(['Z', ' ', '', 'P']);
    expect(warehouse.getStack(1)).toEqual(['Z']);
    expect(warehouse.getStack(2)).toEqual([]);
    expect(warehouse.getStack(3)).toEqual([]);
    expect(warehouse.getLayer(1)).toEqual(['Z', '', '', 'P']);
  });

  it('should be able to move a crate from the top of a stack to another stack', () => {
    const warehouse = new Warehouse();
    warehouse.addLayer(['Z', 'M', 'P']);
    warehouse.addLayer(['N', 'C']);
    warehouse.addLayer(['', 'D']);

    warehouse.moveCrate(2, 1);
    expect(warehouse.getLayer(3)).toEqual(['D', '', '']);
    expect(warehouse.getLayer(2)).toEqual(['N', 'C', '']);
    expect(warehouse.getLayer(1)).toEqual(['Z', 'M', 'P']);
  });

  it('should be able to move multiple crates from the top of a stack to another stack', () => {
    const warehouse = new Warehouse();
    warehouse.addLayer(['Z', 'M', 'P']);
    warehouse.addLayer(['N', 'C']);
    warehouse.addLayer(['', 'D']);

    warehouse.moveCrates(1, 2, 1);
    expect(warehouse.getLayer(3)).toEqual(['D', '', '']);
    expect(warehouse.getLayer(2)).toEqual(['N', 'C', '']);
    expect(warehouse.getLayer(1)).toEqual(['Z', 'M', 'P']);

    warehouse.moveCrates(3, 1, 3);
    expect(warehouse.getLayer(4)).toEqual(['', '', 'Z']);
    expect(warehouse.getLayer(3)).toEqual(['', '', 'N']);
    expect(warehouse.getLayer(2)).toEqual(['', 'C', 'D']);
    expect(warehouse.getLayer(1)).toEqual(['', 'M', 'P']);

    warehouse.moveCrates(2, 2, 1);
    expect(warehouse.getLayer(4)).toEqual(['', '', 'Z']);
    expect(warehouse.getLayer(3)).toEqual(['', '', 'N']);
    expect(warehouse.getLayer(2)).toEqual(['M', '', 'D']);
    expect(warehouse.getLayer(1)).toEqual(['C', '', 'P']);

    warehouse.moveCrates(...[1, 1, 2]);
    expect(warehouse.getLayer(4)).toEqual(['', '', 'Z']);
    expect(warehouse.getLayer(3)).toEqual(['', '', 'N']);
    expect(warehouse.getLayer(2)).toEqual(['', '', 'D']);
    expect(warehouse.getLayer(1)).toEqual(['C', 'M', 'P']);
  });

  it("should be able to tell what's the current highest layer", () => {
    const warehouse = new Warehouse();
    warehouse.addLayer(['Z', 'M', 'P']);
    warehouse.addLayer(['N', 'C']);
    warehouse.addLayer(['', 'D']);

    warehouse.moveCrates(1, 2, 1);
    expect(warehouse.getLayersAmount()).toEqual(3);

    warehouse.moveCrates(3, 1, 3);
    expect(warehouse.getLayersAmount()).toEqual(4);
  });

  it('should be able to be serialized', () => {
    const warehouse = new Warehouse();
    warehouse.addLayer(['Z', 'M', 'P']);
    warehouse.addLayer(['N', 'C']);
    warehouse.addLayer(['', 'D']);

    expect(warehouse.serialize()).toEqual(`    [D]    
[N] [C]    
[Z] [M] [P]`);
  });

  it('should be able to be loaded from a serialized warehouse', () => {
    const serializedWarehouse = `    [D]    
[N] [C]    
[Z] [M] [P]`;

    const warehouse = new Warehouse();
    warehouse.deserialize(serializedWarehouse);
    expect(warehouse.getLayer(3)).toEqual(['', 'D', '']);
    expect(warehouse.getLayer(2)).toEqual(['N', 'C', '']);
    expect(warehouse.getLayer(1)).toEqual(['Z', 'M', 'P']);
  });

  it('should be able to print the crate who is on top of a given stack', () => {
    const serializedWarehouse = `    [D]    
[N] [C]    
[Z] [M] [P]`;

    const warehouse = new Warehouse();
    warehouse.deserialize(serializedWarehouse);

    expect(warehouse.getCrateOnTopOfStack(1)).toEqual('N');
    expect(warehouse.getCrateOnTopOfStack(2)).toEqual('D');
    expect(warehouse.getCrateOnTopOfStack(3)).toEqual('P');
  });

  it('should be able to print the crates who are on top of each stacks', () => {
    const serializedWarehouse = `    [D]    
[N] [C]    
[Z] [M] [P]`;

    const warehouse = new Warehouse();
    warehouse.deserialize(serializedWarehouse);

    expect(warehouse.getCratesOnTopOfStacks()).toEqual(['N', 'D', 'P']);
  });
});
