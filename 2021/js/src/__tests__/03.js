import { getLinesOfPuzzleInput } from '../../utils/parseInput.js';
import { firstStar, secondStar } from '../03.js';
import ava from 'ava';

const input = await getLinesOfPuzzleInput('03');

ava('Day 03 - First Star', (t) => {
	t.is(firstStar(input), 0);
});

ava('Day 03 - Second Star', (t) => {
	t.is(secondStar(input), 0);
});
