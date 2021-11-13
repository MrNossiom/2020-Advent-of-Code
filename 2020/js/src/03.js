function* nextSlopePositon(length, width, step) {
	let position = 0;
	for (let index = 0; index < length; index++) {
		yield position;

		position += step;
		if (position / (width - 1) > 1) position -= width;
	}
}

export const checkSlope = (input, rightStep, downStep) => {
	let counter = 0;
	const position = nextSlopePositon(input.length, input[0].length, rightStep);

	for (let index = 0; index < input.length; index += downStep) {
		const nextPosition = position.next();

		if (nextPosition.done) return -1;
		if (input[index][nextPosition.value] === '#') {
			counter++;
		}
	}

	return counter;
};

export const firstStar = (input) => {
	return checkSlope(input, 3, 1);
};

export const secondStar = (input) => {
	return (
		checkSlope(input, 1, 1) *
		checkSlope(input, 3, 1) *
		checkSlope(input, 5, 1) *
		checkSlope(input, 7, 1) *
		checkSlope(input, 1, 2)
	);
};

// Found an other small implementation

const countNumberOfTreesEncountered = (grid, numRight, numDown) => {
	let treesEncountered = 0;

	for (let row = 0; row < grid.length; row += numDown) {
		const col = ((row / numDown) * numRight) % grid[0].length;
		if (grid[row][col] == '#') treesEncountered++;
	}

	return treesEncountered;
};

export const firstStar_2impl = (input) => {
	return countNumberOfTreesEncountered(input, 3, 1);
};

export const secondStar_2impl = (input) => {
	return (
		countNumberOfTreesEncountered(input, 1, 1) *
		countNumberOfTreesEncountered(input, 3, 1) *
		countNumberOfTreesEncountered(input, 5, 1) *
		countNumberOfTreesEncountered(input, 7, 1) *
		countNumberOfTreesEncountered(input, 1, 2)
	);
};
