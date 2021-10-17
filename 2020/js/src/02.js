import { second } from '../input.json';

function parseLine(line) {
	const parts = line.split(' ');

	const [firstNumber, secondNumber] = parts[0]
		.split('-')
		.map((string) => parseInt(string, 10));
	const letter = parts[1].charAt(0);
	const password = parts[2];

	return [firstNumber, secondNumber, letter, password];
}

function isValidPasswordFirstPolicy(line) {
	const [min, max, letter, password] = parseLine(line);

	const numberOfLetterInPass = [...password.matchAll(letter)].length;

	return numberOfLetterInPass >= min && numberOfLetterInPass <= max;
}

function isValidPasswordSecondPolicy(line) {
	const [firstNumber, secondNumber, letter, password] = parseLine(line);

	return (
		(password.charAt(firstNumber - 1) === letter) ^
		(password.charAt(secondNumber - 1) === letter)
	);
}

let firstPolicyCount = 0;
for (pass of second) {
	if (isValidPasswordFirstPolicy(pass)) firstPolicyCount += 1;
}

let secondPolicyCount = 0;
for (pass of second) {
	if (isValidPasswordSecondPolicy(pass)) secondPolicyCount += 1;
}

console.log('First star:', firstPolicyCount);
console.log('Second star:', secondPolicyCount);
