import { getLinesOfPuzzleInput } from '../utils/parseInput.js';

const input = getLinesOfPuzzleInput('01');

// O(n^2) function
export const firstStar = (numbers) => {
	for (i in numbers) {
		for (ii in numbers) {
			if (numbers[i] + numbers[ii] === 2020) {
				return `${numbers[i]} + ${numbers[ii]}`;
			}
		}
	}
};

// O(n^3) function
export const secondStar = (numbers) => {
	for (i in numbers) {
		for (ii in numbers) {
			for (iii in numbers) {
				if (numbers[i] + numbers[ii] + numbers[iii] === 2020) {
					return `${numbers[i]} + ${numbers[ii]} + ${numbers[iii]}`;
				}
			}
		}
	}
};

// Test
console.log('First star:', firstStar(input));
console.log('Second star:', secondStar(input));
