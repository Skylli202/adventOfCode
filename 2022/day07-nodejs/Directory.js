import { File } from "./File.js"

export class Directory {
	#name
	#parent
	#directories
	#files
	constructor(name) {
		this.#name = name
		this.#parent = undefined
		this.#directories = []
		this.#files = []
	}

	toString() {
		return `- ${this.#name} (dir, size=${this.computeSize()})`
	}

	/**
	 * @returns {string}
	 */
	prettyPrint() {
		let tmp = this.toString()

		/**
		 * @param {string} accumulator
		 * @param {Directory} directory
		 */
		const directoryReduceCallback = (accumulator, directory) => {
			// Add a new line
			accumulator += '\n'

			// Print directory properties, subdirectories and files
			accumulator += directory
				.prettyPrint()
				.split('\n')
				.map(value => { return `  ${value}` })
				.join('\n')

			return accumulator
		}

		tmp = this.#directories.reduce(directoryReduceCallback, tmp)

		/**
		 * @param {string} accumulator
		 * @param {File} file
		 */
		const filesReduceCallback = (accumulator, file) => {
			accumulator += `\n`

			accumulator += `  ${file.toString()}`

			return accumulator
		}
		tmp = this.#files.reduce(filesReduceCallback, tmp)

		return tmp
	}

	/**
	 * @returns {number} return the size (in bytes) of the directory.
	 */
	computeSize() {
		// TODO: implement this function
		return 0
	}

	/**
	* @param {Directory} directory 
	*/
	addDir(directory) {
		this.#directories.push(directory)
		directory.#parent = this
	}

	/**
	* @param {File} file 
	*/
	addFile(file) {
		this.#files.push(file)
	}

	/**
	 * @param {Directory} parent 
	 */
	setParent(parent) { this.#parent = parent }

	/**
	 * @returns {Directory} 
	 */
	getParent() { return this.#parent }

}
