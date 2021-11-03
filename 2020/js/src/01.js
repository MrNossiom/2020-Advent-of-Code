// O(n^2) function
export const firstStar = (numbers) => {
	for (const i in numbers) {
		for (const ii in numbers) {
			if (numbers[i] + numbers[ii] === 2020) {
				return `${numbers[i]} + ${numbers[ii]}`;
			}
		}
	}
};

// O(n^3) function
export const secondStar = (numbers) => {
	for (const i in numbers) {
		for (const ii in numbers) {
			for (const iii in numbers) {
				if (numbers[i] + numbers[ii] + numbers[iii] === 2020) {
					return `${numbers[i]} + ${numbers[ii]} + ${numbers[iii]}`;
				}
			}
		}
	}
};
