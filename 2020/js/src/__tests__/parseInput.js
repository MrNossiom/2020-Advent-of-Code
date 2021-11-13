import {
	getBlocksOfPuzzleInput,
	getLinesOfPuzzleInput,
	getPuzzleInput,
} from '../../utils/parseInput.js';
import ava from 'ava';

const SAMPLE_TEXT = `Hello, World!
I'm a test sample to ensure the parser works properly.

Here is an space before this line.
This line is the end of the block`;

ava('Correctly read the sample input', async (t) => {
	t.deepEqual(await getPuzzleInput('sample'), SAMPLE_TEXT);
});

ava('Correctly parse line by line the sample input', async (t) => {
	t.deepEqual(await getLinesOfPuzzleInput('sample'), SAMPLE_TEXT.split('\n'));
});

ava('Correctly parse blocks of the sample input', async (t) => {
	t.deepEqual(
		await getBlocksOfPuzzleInput('sample'),
		SAMPLE_TEXT.split('\n\n')
	);
});
