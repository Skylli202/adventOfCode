import { Choice, RoundResult } from './RockPaperScissors';

export interface ParseOptions {
	sep?: string;
	transform?: Record<string, number>;
}

export const parse = (data: string, options: ParseOptions = {}) => {
	const lines = data.split(/\r\n|\n|\r/);
	if (options.sep === undefined) return lines;
	const splitedLines: String[][] = lines.map((value) =>
		value.split(options.sep as string)
	);
	if (options.transform === undefined) return splitedLines;

	const transformedResult: number[][] = [];
	for (const line of splitedLines) {
		const tmp: number[] = [];
		for (const el of line) {
			for (const [key, value] of Object.entries(PUZZLE_CHOICE_MAP)) {
				console.log(`key:${key}, value:${value}`);

				if (el === key) {
					tmp.push(value);
				}
			}
		}
		transformedResult.push(tmp);
	}
	return transformedResult;
};

export const PUZZLE_CHOICE_MAP = {
	A: Choice.ROCK,
	X: Choice.ROCK,
	B: Choice.PAPER,
	Y: Choice.PAPER,
	C: Choice.SCISSORS,
	Z: Choice.SCISSORS,
};

export const PUZZLE_CHOICE_MAP2 = {
	A: Choice.ROCK,
	X: 0,
	B: Choice.PAPER,
	Y: 3,
	C: Choice.SCISSORS,
	Z: 6,
};
