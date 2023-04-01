/**
 * @param {string} craneOrder
 * @returns {number[]}
 */
export function parseCraneOrder(craneOrder) {
  const tmp = craneOrder.replaceAll(/move|from|to|\s/gi, '');
  return tmp.split('').map((value) => Number(value));
}

/**
 * Parse a string representative of a layer of crates in a stack.
 * @param {string} crateLayer
 * @return {string[]}
 */
export function parseLayer(crateLayer) {
  const crates = [];
  let i = 1;
  while (i <= crateLayer.length) {
    crates.push(crateLayer.charAt(i) === ' ' ? '' : crateLayer.charAt(i));
    i += 4;
  }
  return crates;
}
