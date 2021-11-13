import { getLinesOfPuzzleInput } from '../../utils/parseInput.js';
import {
	firstStar,
	firstStar_2impl,
	secondStar,
	secondStar_2impl,
} from '../03.js';
import ava from 'ava';

const treeMap = await getLinesOfPuzzleInput('03');

ava('Day 03 - First Star', (t) => {
	t.is(firstStar(treeMap), 268);
});

ava('Day 03 - Second Star', (t) => {
	t.is(secondStar(treeMap), 3093068400);
});

ava('Day 03 - First Star - Other implementation', (t) => {
	t.is(firstStar_2impl(treeMap), 268);
});

ava('Day 03 - Second Star - Other implementation', (t) => {
	t.is(secondStar_2impl(treeMap), 3093068400);
});
