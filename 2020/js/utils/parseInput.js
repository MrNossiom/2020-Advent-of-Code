import { readFile } from 'node:fs/promises';
import path from 'node:path';

export const getLinesOfInput = (input) => {
	return input.split('\n');
};

export const getLinesOfPuzzleInput = async (day) => {
	const content = await readFile(
		path.resolve(process.cwd(), `../../inputs/${day}.txt`),
		{ encoding: 'utf-8' }
	);

	return getLinesOfInput(content);
};
