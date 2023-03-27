import { readFileSync } from 'fs';
import { PUZZLE_CHOICE_MAP, PUZZLE_CHOICE_MAP2, parse } from './puzzleParser';
import { Choice, RockPaperScissorsGame } from './RockPaperScissors';
import { Player } from './Player';

const content = readFileSync('./src/assets/puzzle-input.txt', {
	encoding: 'utf-8',
});

const roundChoices = parse(content, {
	sep: ' ',
	transform: PUZZLE_CHOICE_MAP,
}) as number[][];

const elfChoices: number[] = [];
const playerChoices: number[] = [];

for (const roundChoice of roundChoices) {
	elfChoices.push(roundChoice[0]);
	playerChoices.push(roundChoice[1]);
}

const player = new Player(playerChoices);
for (const elfChoice of elfChoices) {
	player.playAgainst(elfChoice);
}
console.log(player.getScore());

const roundChoices2 = parse(content, {
	sep: ' ',
	transform: PUZZLE_CHOICE_MAP2,
}) as number[][];

const elfChoices2: number[] = [];
const playerChoices2: number[] = [];

let i = 0;
for (const roundChoice of roundChoices2) {
	console.log(roundChoice);

	elfChoices2.push(roundChoice[0]);
	// playerChoices2.push(roundChoice[1]);
	console.log(
		RockPaperScissorsGame.getPlayForOutcome(roundChoice[0], roundChoice[1])
	);

	playerChoices2.push(0);
	i++;
	if (i > 2) {
		break;
	}
}
console.log(elfChoices2, playerChoices2);

const player2 = new Player(playerChoices2);
for (const elfChoice of elfChoices2) {
	player2.playAgainst(elfChoice);
}
console.log(player2.getScore());

// const MAP_PUZZLE_TO_CHOICE = {
// 	A: 1,
// 	B: 2,
// 	C: 3,
// };

// const MAP_PUZZLE_TO_ROUNDRESULT = {
// 	X: 0,
// 	Y: 3,
// 	Z: 6,
// };

// const elfChoices2 = [];
// const playerChoices2 = [];
// let i = 0;
// for (const line of content.split(/\r|\n|\r\n/)) {
// 	const [elf, outcome] = line.split(' ');
// 	elfChoices2.push(MAP_PUZZLE_TO_CHOICE[elf as 'A' | 'B' | 'C']);
// 	playerChoices2.push(
// 		RockPaperScissorsGame.getPlayForOutcome(
// 			MAP_PUZZLE_TO_CHOICE[elf as 'A' | 'B' | 'C'],
// 			MAP_PUZZLE_TO_ROUNDRESULT[outcome as 'X' | 'Y' | 'Z']
// 		)
// 	);
// }

// const player2 = new Player(playerChoices2);
// for (const elfChoice of elfChoices2) {
// 	player2.playAgainst(elfChoice);
// }
// console.log(player2.getScore());
