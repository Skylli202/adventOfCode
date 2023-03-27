import { expect, it } from 'vitest';
import { Player } from './Player';
import { Choice } from './RockPaperScissors';

it('should have a score of 15 after playing Paper, Rock, Scissors against Rock, Paper, Scissors', () => {
	const choices = [Choice.PAPER, Choice.ROCK, Choice.SCISSORS];
	const player = new Player(choices);
	player.playAgainst(Choice.ROCK);
	expect(player.getScore()).toEqual(8);
	player.playAgainst(Choice.PAPER);
	expect(player.getScore()).toEqual(8 + 1);
	player.playAgainst(Choice.SCISSORS);
	expect(player.getScore()).toEqual(8 + 1 + 6);
});
