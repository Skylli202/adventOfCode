export class File {
	#name
	#size
	constructor(name, size) {
		this.#name = name
		this.#size = size
	}

	toString() {
		return `- ${this.#name} (file, size=${this.#size})`
	}
}
