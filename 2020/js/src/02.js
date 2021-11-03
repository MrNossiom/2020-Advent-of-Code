import { xor } from '../utils/logicalDoors.js';

const parseLine = (line) => {
	const parts = line.split(' ');

	const [firstNumber, secondNumber] = parts[0]
		.split('-')
		.map((string) => parseInt(string, 10));
	const letter = parts[1].charAt(0);
	const password = parts[2];

	return [firstNumber, secondNumber, letter, password];
};

const isValidPasswordFirstPolicy = (line) => {
	const [min, max, letter, password] = parseLine(line);

	const numberOfLetterInPass = [...password.matchAll(new RegExp(letter, 'g'))]
		.length;

	return numberOfLetterInPass >= min && numberOfLetterInPass <= max;
};

const isValidPasswordSecondPolicy = (line) => {
	const [firstNumber, secondNumber, letter, password] = parseLine(line);

	return xor(
		password.charAt(firstNumber - 1) === letter,
		password.charAt(secondNumber - 1) === letter
	);
};

export const firstStar = (passwords) => {
	let validPolicyCount = 0;

	for (const password of passwords) {
		if (isValidPasswordFirstPolicy(password)) validPolicyCount++;
	}

	return validPolicyCount;
};

export const secondStar = (passwords) => {
	let validPolicyCount = 0;

	for (const password of passwords) {
		if (isValidPasswordSecondPolicy(password)) validPolicyCount += 1;
	}

	return validPolicyCount;
};
