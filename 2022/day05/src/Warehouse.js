import { parseLayer } from './parser.js';

export class Warehouse {
  /**
   * @private
   * @property {string[][]} crateStacks
   */
  #crateStacks;

  constructor() {
    this.#crateStacks = [];
  }

  /**
   * Move the crate, which is on top of the stack "from", to the stack in "to" position.
   * Warning, from and to are position (not index).
   * @param {number} from
   * @param {number} to
   * @return {this}
   */
  moveCrate(from, to) {
    const movedCrate = this.getStack(from).pop();
    this.getStack(to).push(movedCrate);
    return this;
  }

  /**
   * Move the given amount of crates, which are on top of the stack "from", to the stack in "to" position.
   * Warning, from and to are position (not index).
   * @param {number} amount
   * @param {number} from
   * @param {number} to
   * @return {this}
   */
  moveCrates(amount, from, to) {
    for (let i = 0; i < amount; i++) {
      this.moveCrate(from, to);
    }
    return this;
  }
  /**
   * Move the given amount of crates, which are on top of the stack "from", to the stack in "to" position. But it does
   * maintain the order to the taken crates.
   * Warning, from and to are position (not index).
   * @param {number} amount
   * @param {number} from
   * @param {number} to
   * @return {this}
   */
  moveCratesConservatively(amount, from, to) {
    const movedCrates = [];
    for (let i = 0; i < amount; i++) {
      const movedCrate = this.getStack(from).pop();
      movedCrates.unshift(movedCrate);
    }
    movedCrates.forEach((movedCrate) => {
      this.addCrate(to, movedCrate);
    });
    return this;
  }

  /**
   * Add the given crate on top of the stack at the given stackPosition.
   * @param {number} stackPosition
   * @param {string} crate
   * @return this
   */
  addCrate(stackPosition, crate) {
    const index = stackPosition - 1;
    if (this.#crateStacks[index] === undefined) {
      this.#crateStacks.push([]);
    }
    this.#crateStacks[index].push(crate);
    return this;
  }

  /**
   * Add a layer of crates to the warehouse.
   * @param {string[]} layer
   * @return {this}
   */
  addLayer(layer) {
    for (let i = 0; i < layer.length; i++) {
      if (/[A-Z]/.test(layer[i])) {
        this.addCrate(i + 1, layer[i]);
      }
    }
    return this;
  }

  /**
   * Return the layer of crate at the given position (position start at 1).
   * If a stack does not have any crate on this layer, it will return an empty string for this crate.
   * @param {number} position
   * @return {string[]}
   */
  getLayer(position) {
    const layer = [];
    for (let i = 0; i < this.#crateStacks.length; i++) {
      const index = position - 1;
      if (this.#crateStacks[i][index] === undefined) {
        layer.push('');
      } else {
        layer.push(this.#crateStacks[i][index]);
      }
    }
    return layer;
  }

  /**
   * Return the stack of the given position (position start at 1).
   * @param {number} position
   * @return {string[]}
   */
  getStack(position) {
    return this.#crateStacks[position - 1];
  }

  /**
   * Return the amount of layers currently in the warehouse.
   * @return {number}
   */
  getLayersAmount() {
    return this.#crateStacks.reduce(
      (accumulator, stack) =>
        accumulator < stack.length ? stack.length : accumulator,
      0
    );
  }

  /**
   * Return the crate who is on the top of the given stack position (position start at 1).
   * @param {number} position
   * @return {string}
   */
  getCrateOnTopOfStack(position) {
    return this.getStack(position).slice(-1).join('');
  }

  /**
   * Return the crates who are on top of each stacks of the warehouse.
   * @return {string[]}
   */
  getCratesOnTopOfStacks() {
    const result = [];
    for (let i = 0; i < this.#crateStacks.length; i++) {
      result.push(this.getCrateOnTopOfStack(i + 1));
    }
    return result;
  }

  /**
   * Load the warehouse from the serialized warehouse given in parameter.
   * @param {string} serializedWarehouse
   * @return {this}
   */
  deserialize(serializedWarehouse) {
    const serializedLayers = serializedWarehouse.split(/\r\n|\n/);
    for (let i = serializedLayers.length - 1; i >= 0; i--) {
      const layer = parseLayer(serializedLayers[i]);
      this.addLayer(layer);
    }
    return this;
  }

  /**
   * Transform the warehouse into a string. For display purpose or exports (over plain file or network)
   * @return {string}
   */
  serialize() {
    return this.toString();
  }

  /**
   * Transform the warehouse into a string, for display and reading purposes.
   * @return {string}
   */
  toString() {
    let result = '';

    for (let i = this.getLayersAmount(); i > 0; i--) {
      if (result !== '') {
        result += '\n';
      }
      result += this.getLayer(i)
        .map((crate) => {
          return crate === '' ? '   ' : `[${crate}]`;
        })
        .join(' ');
    }

    return result;
  }
}
