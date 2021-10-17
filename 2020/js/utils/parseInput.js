import { readFile } from 'node:fs/promises';

export const getLinesOfInput = (input) => {
	return input.split('\n');
};

export const getLinesOfPuzzleInput = (day) => {
	const content = await readFile(
		path.resolve(__dirname, `../../inputs/${day}.txt`)
	);

	return getLinesOfInput(content);
};
