import { Choice, RockPaperScissorsGame } from './RockPaperScissors';

export class Player {
	private score: number;
	private round: number;
	private roundChoices: Choice[];

	constructor(choices: Choice[]) {
		this.score = 0;
		this.round = 0;
		this.roundChoices = choices;
	}

	playAgainst(choice: Choice) {
		this.score += RockPaperScissorsGame.computeScore(
			this.roundChoices[this.round],
			RockPaperScissorsGame.computeRound(this.roundChoices[this.round], choice)
		);
		this.round += 1;
	}

	getScore(): number {
		return this.score;
	}
}
