export enum Choice {
	ROCK = 1,
	PAPER = 2,
	SCISSORS = 3,
}

export enum RoundResult {
	WIN = 6,
	DRAW = 3,
	LOOSE = 0,
}

export class RockPaperScissorsGame {
	/**
	 * Return the result for the given choices.
	 * The result is from the point of view of the first choice.
	 *
	 * @example
	 *  computeRound(ROCK, SCISSORS): WIN
	 *  computeRound(PAPER, SCISSORS): LOOSE
	 *  computeRound(PAPER, ROCK): DRAW
	 * @param choice1
	 * @param choice2
	 * @returns
	 */
	static computeRound(choice1: Choice, choice2: Choice): RoundResult {
		switch (choice1) {
			case Choice.ROCK: {
				switch (choice2) {
					case Choice.SCISSORS:
						return RoundResult.WIN;
					case Choice.ROCK:
						return RoundResult.DRAW;
					case Choice.PAPER:
						return RoundResult.LOOSE;
				}
			}

			case Choice.PAPER: {
				switch (choice2) {
					case Choice.ROCK:
						return RoundResult.WIN;
					case Choice.PAPER:
						return RoundResult.DRAW;
					case Choice.SCISSORS:
						return RoundResult.LOOSE;
				}
			}

			case Choice.SCISSORS: {
				switch (choice2) {
					case Choice.PAPER:
						return RoundResult.WIN;
					case Choice.SCISSORS:
						return RoundResult.DRAW;
					case Choice.ROCK:
						return RoundResult.LOOSE;
				}
			}
		}
	}

	/**
	 * Return the score gain base upon what have been played (choice)
	 * and the result of the round (roundResult).
	 * @example
	 *  computeScore(PAPER, WIN): 8
	 *  computeScore(ROCK, LOOSE): 1
	 *  computeScore(SCISSORS, DRAW): 6
	 * @param choice
	 * @param roundResult
	 * @returns
	 */
	static computeScore(choice: Choice, roundResult: RoundResult): number {
		return choice + roundResult;
	}

	static getPlayForOutcome(
		opponentChoice: Choice,
		desiredOutcome: RoundResult
	): Choice {
		switch (opponentChoice) {
			case Choice.ROCK:
				switch (desiredOutcome) {
					case RoundResult.WIN:
						return Choice.PAPER;
					case RoundResult.DRAW:
						return Choice.ROCK;
					case RoundResult.LOOSE:
						return Choice.SCISSORS;
				}
			case Choice.PAPER:
				switch (desiredOutcome) {
					case RoundResult.WIN:
						return Choice.SCISSORS;
					case RoundResult.DRAW:
						return Choice.PAPER;
					case RoundResult.LOOSE:
						return Choice.ROCK;
				}
			case Choice.SCISSORS:
				switch (desiredOutcome) {
					case RoundResult.WIN:
						return Choice.ROCK;
					case RoundResult.DRAW:
						return Choice.SCISSORS;
					case RoundResult.LOOSE:
						return Choice.PAPER;
				}
		}
	}
}
