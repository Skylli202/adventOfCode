import { File } from "./File.js";
import { Directory } from "./Directory.js";
import { readFileSync } from "fs"
import { error, log } from "console";

const args = process.argv.slice(2)

const file = readFileSync(args[0], { encoding: 'utf-8' })
const lines = file.split(`\n`)

const rootDir = new Directory('/')
/** @type {Directory} */
let currentDirectory = rootDir

lines.forEach(line => {
    // console.log(`line=${line}`);
    if (line.charAt(0) === '$') {
        switch (line.slice(2, 4)) {
            case 'cd': {
                const target = line.slice(5).trim()
                switch (target) {
                    case '..': {
                        if (currentDirectory === null) {
                            throw new Error('Cannot access parent directory because currentDirectory is undefined.')
                            process.exit(1)
                        }

                        if (currentDirectory.getParent() === null) {
                            console.error(currentDirectory.prettyPrint())
                            throw new error('Cannot access parent of currentDirectory, seems like currentDirectory is root.')
                            process.exit(1)
                        }

                        currentDirectory = currentDirectory.getParent()
                        break
                    }
                    case '/': {
                        currentDirectory = rootDir
                        breakrootDirrootDir
                    }
                    default: {
                        const newSubDir = new Directory(target)
                        currentDirectory.addDir(newSubDir)
                        currentDirectory = newSubDir
                        break
                    }
                }
                break
            }
            case 'ls': {
                
                break
            }
            default: {
                console.log('oops.. Unknown command parsed');
                console.debug(line.slice(2, 4))
            }
        }
    } else {

    }
    // console.log(`current directory:\n${currentDirectory.toString()}`);
});

console.log(rootDir.prettyPrint())
