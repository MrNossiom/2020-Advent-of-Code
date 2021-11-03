function* nextSlopePositon(length, width) {
	let position = 0;
	for (let index = 0; index < length; index++) {
		yield position;

		position += 3;
		if (position / width > 1) position -= width;
	}
}

export const firstStar = (input) => {
	let counter = 0;
	const position = nextSlopePositon(input.length, input[0].length);

	for (const line of input) {
		const nextPosition = position.next();

		if (nextPosition.done) return -1;
		if (line.charAt(nextPosition.value - 1) === '#') counter++;
	}

	return counter;
};

export const secondStar = (input) => {};
