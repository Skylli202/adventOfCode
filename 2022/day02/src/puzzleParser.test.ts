import { expect, it, test } from 'vitest';
import { parse, ParseOptions, PUZZLE_CHOICE_MAP } from './puzzleParser';
import { Choice } from './RockPaperScissors';

test.each([
	['1\n2\n3\n4\n5\n6'],
	['1\r2\r3\r4\r5\r6'],
	['1\r\n2\r\n3\r\n4\r\n5\r\n6'],
])(
	'%# - should properly split lines, without knowing the EOF characters',
	(input) => {
		const result = parse(input);
		expect(result.length).toEqual(6);
	}
);

it.each([
	['1 1\n2 2\n3 3\n4 4\n5 5\n6 6', ' '],
	['1-1\r2-2\r3-3\r4-4\r5-5\r6-6', '-'],
	['1~1\r\n2~2\r\n3~3\r\n4~4\r\n5~5\r\n6~6', '~'],
])(
	'%# - if a sep is given in options, split each line with this sep and return an array of arrays',
	(input: string, separator: string) => {
		const results = parse(input, { sep: separator });
		expect(Array.isArray(results)).toBe(true);

		for (const result of results) {
			expect(Array.isArray(result)).toBe(true);
			expect(result.length).toEqual(2);
		}
	}
);

it.each([
	[
		'A Y',
		{ sep: ' ', transform: PUZZLE_CHOICE_MAP },
		[[Choice.ROCK, Choice.PAPER]],
	],
	// [
	// 	'B X',
	// 	{ sep: ' ', transform: PUZZLE_CHOICE_MAP },
	// 	[[Choice.PAPER, Choice.ROCK]],
	// ],
	// [
	// 	'C Z',
	// 	{ sep: ' ', transform: PUZZLE_CHOICE_MAP },
	// 	[[Choice.SCISSORS, Choice.SCISSORS]],
	// ],
])(
	'%# - it should parse, separate and transform when options for sep and transform is given',
	(input: string, options: ParseOptions, expectedResults: Choice[][]) => {
		const results = parse(input, options);
		expect(results).toEqual(expectedResults);
	}
);
