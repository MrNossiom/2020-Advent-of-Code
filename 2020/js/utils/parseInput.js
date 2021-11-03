import { readFile } from 'node:fs/promises';
import path from 'node:path';

export const getLinesOfPuzzleInput = async (day) => {
	const lines = (await getPuzzleInput(day)).split('\n');

	return lines;
};

export const getBlocksOfPuzzleInput = async (day) => {
	const lines = (await getPuzzleInput(day)).split('\n\n');

	return lines;
};

export const getPuzzleInput = async (day) => {
	const content = await readFile(
		path.resolve(process.cwd(), `../inputs/${day}.txt`),
		{ encoding: 'utf-8' }
	);

	return content;
};
