import input from '../input.json';

// O(n^2) function
function firstStar(numbers) {
	for (i in numbers) {
		for (ii in numbers) {
			if (numbers[i] + numbers[ii] === 2020) {
				return `${numbers[i]} + ${numbers[ii]}`;
			}
		}
	}
}

// O(n^3) function
function secondStar(numbers) {
	for (i in numbers) {
		for (ii in numbers) {
			for (iii in numbers) {
				if (numbers[i] + numbers[ii] + numbers[iii] === 2020) {
					return `${numbers[i]} + ${numbers[ii]} + ${numbers[iii]}`;
				}
			}
		}
	}
}

// Test
console.log('First star: ', firstStar(input));
console.log('Second star: ', secondStar(input));
