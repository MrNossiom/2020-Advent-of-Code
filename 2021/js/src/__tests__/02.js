import { getLinesOfPuzzleInput } from '../../utils/parseInput.js';
import { firstStar, secondStar } from '../02.js';
import ava from 'ava';

const input = await getLinesOfPuzzleInput('02');

ava('Day 02 - First Star', (t) => {
	t.is(firstStar(input), 1604850);
});

ava('Day 02 - Second Star', (t) => {
	t.is(secondStar(input), 1685186100);
});
