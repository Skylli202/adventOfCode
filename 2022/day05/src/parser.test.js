import { describe, expect, it } from 'vitest';
import { parseCraneOrder, parseLayer } from './parser.js';

describe('parseCraneOrder', () => {
  it.each([
    ['move 1 from 7 to 4', [1, 7, 4]],
    ['move 1 from 6 to 2', [1, 6, 2]],
    ['move 5 from 9 to 4', [5, 9, 4]],
    ['move 2 from 2 to 8', [2, 2, 8]],
    ['move 2 from 2 to 6', [2, 2, 6]],
    ['move 3 from 3 to 7', [3, 3, 7]],
    ['move 3 from 7 to 1', [3, 7, 1]],
    ['move 1 from 9 to 4', [1, 9, 4]],
  ])('should properly parse crane orders', (craneOrder, expectParsing) => {
    expect(parseCraneOrder(craneOrder)).toEqual(expectParsing);
  });
});

describe('parseLayer', () => {
  it.each([
    ['    [D]    ', ['', 'D', '']],
    ['[N] [C]    ', ['N', 'C', '']],
    ['[Z] [M] [P]', ['Z', 'M', 'P']],
  ])(
    'should properly parse crate from stackLayer',
    (stackLayer, expectedCrates) => {
      expect(parseLayer(stackLayer)).toEqual(expectedCrates);
    }
  );
});
