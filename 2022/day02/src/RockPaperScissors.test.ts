import { describe, expect, it } from 'vitest';
import {
	Choice,
	RockPaperScissorsGame,
	RoundResult,
} from './RockPaperScissors';

describe('RockPaperScissorsGame.computeRound() : ', () => {
	describe('ROCK should ', () => {
		it('WIN against SCISSORS', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.ROCK, Choice.SCISSORS)
			).toBe(RoundResult.WIN);
		});

		it('DRAW against ROCK', () => {
			expect(RockPaperScissorsGame.computeRound(Choice.ROCK, Choice.ROCK)).toBe(
				RoundResult.DRAW
			);
		});

		it('LOOSE against PAPER', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.ROCK, Choice.PAPER)
			).toBe(RoundResult.LOOSE);
		});
	});

	describe('PAPER should ', () => {
		it('WIN against ROCK', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.PAPER, Choice.ROCK)
			).toBe(RoundResult.WIN);
		});

		it('DRAW against PAPER', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.PAPER, Choice.PAPER)
			).toBe(RoundResult.DRAW);
		});

		it('LOOSE against SCISSORS', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.PAPER, Choice.SCISSORS)
			).toBe(RoundResult.LOOSE);
		});
	});

	describe('SCISSORS should ', () => {
		it('WIN against PAPER', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.SCISSORS, Choice.PAPER)
			).toBe(RoundResult.WIN);
		});

		it('DRAW against SCISSORS', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.SCISSORS, Choice.SCISSORS)
			).toBe(RoundResult.DRAW);
		});

		it('LOOSE against SCISSORS', () => {
			expect(
				RockPaperScissorsGame.computeRound(Choice.SCISSORS, Choice.ROCK)
			).toBe(RoundResult.LOOSE);
		});
	});
});

describe('RockPaperScissorsGame.computeScore() : ', () => {
	describe('ROCK', () => {
		it('should return ? when it WIN', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.ROCK, RoundResult.WIN)
			).toEqual(1 + 6);
		});
		it('should return ? when it DRAW', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.ROCK, RoundResult.DRAW)
			).toEqual(1 + 3);
		});
		it('should return ? when it LOOSE', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.ROCK, RoundResult.LOOSE)
			).toEqual(1 + 0);
		});
	});
	describe('PAPER', () => {
		it('should return ? when it WIN', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.PAPER, RoundResult.WIN)
			).toEqual(2 + 6);
		});
		it('should return ? when it DRAW', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.PAPER, RoundResult.DRAW)
			).toEqual(2 + 3);
		});
		it('should return ? when it LOOSE', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.PAPER, RoundResult.LOOSE)
			).toEqual(2 + 0);
		});
	});
	describe('SCISSORS', () => {
		it('should return ? when it WIN', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.SCISSORS, RoundResult.WIN)
			).toEqual(3 + 6);
		});
		it('should return ? when it DRAW', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.SCISSORS, RoundResult.DRAW)
			).toEqual(3 + 3);
		});
		it('should return ? when it LOOSE', () => {
			expect(
				RockPaperScissorsGame.computeScore(Choice.SCISSORS, RoundResult.LOOSE)
			).toEqual(3 + 0);
		});
	});
});

describe('RockPaperScissorsGame.getPlayForOutcome()', () => {
	describe('if opponent play ROCK', () => {
		it('and desired outcome is WIN', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(Choice.ROCK, RoundResult.WIN)
			).toBe(Choice.PAPER);
		});

		it('and desired outcome is DRAW', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(Choice.ROCK, RoundResult.DRAW)
			).toBe(Choice.ROCK);
		});

		it('and desired outcome is LOOSE', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(Choice.ROCK, RoundResult.LOOSE)
			).toBe(Choice.SCISSORS);
		});
	});

	describe('if opponent play PAPER', () => {
		it('and desired outcome is WIN', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(Choice.PAPER, RoundResult.WIN)
			).toBe(Choice.SCISSORS);
		});

		it('and desired outcome is DRAW', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(Choice.PAPER, RoundResult.DRAW)
			).toBe(Choice.PAPER);
		});

		it('and desired outcome is LOOSE', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(Choice.PAPER, RoundResult.LOOSE)
			).toBe(Choice.ROCK);
		});
	});

	describe('if opponent play SCISSORS', () => {
		it('and desired outcome is WIN', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(
					Choice.SCISSORS,
					RoundResult.WIN
				)
			).toBe(Choice.ROCK);
		});

		it('and desired outcome is DRAW', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(
					Choice.SCISSORS,
					RoundResult.DRAW
				)
			).toBe(Choice.SCISSORS);
		});

		it('and desired outcome is LOOSE', () => {
			expect(
				RockPaperScissorsGame.getPlayForOutcome(
					Choice.SCISSORS,
					RoundResult.LOOSE
				)
			).toBe(Choice.PAPER);
		});
	});
});
