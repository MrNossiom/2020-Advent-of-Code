import { getLinesOfPuzzleInput } from '../../utils/parseInput.js';
import { firstStar, secondStar } from '../02.js';
import ava from 'ava';

const passwords = await getLinesOfPuzzleInput('02');

ava('Day 02 - First Star', (t) => {
	t.is(firstStar(passwords), 447);
});

ava('Day 02 - Second Star', (t) => {
	t.is(secondStar(passwords), 249);
});
