export const firstStar = (numbers) => {
	let previous = 0;
	let numberOfLargerMeasurements = 0;

	for (const index in numbers) {
		if (numbers[index] > previous) {
			numberOfLargerMeasurements++;
		}

		previous = numbers[index];
	}

	return numberOfLargerMeasurements - 1;
};

export const secondStar = (numbers) => {
	let previous = 0;

	for (const i in numbers) {
	}
};
