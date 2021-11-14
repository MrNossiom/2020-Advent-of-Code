const parsePassports = (input) => {
	const fields = input.split(/ |\n/gmu);

	const parsedPassport = {};

	for (const field in fields) {
		const [key, value] = fields[field].split(':');
		parsedPassport[key] = value;
	}

	return parsedPassport;
};

const validatePassport = (passport) => {
	for (const requiredFieldName of [
		'byr',
		'iyr',
		'eyr',
		'hgt',
		'hcl',
		'ecl',
		'pid',
		// 'cid',
	]) {
		if (!passport[requiredFieldName]) {
			return false;
		}
	}

	return true;
};

const validatePassport2 = (passport) => {
	for (const [name, value] of Object.entries(passport)) {
		const parsedValue = parseInt(value, 10);

		if (name === 'byr' && !(parsedValue > 1920 && parsedValue <= 2002)) {
			return false;
		} else if (
			name === 'iyr' &&
			!(parsedValue > 2010 && parsedValue <= 2020)
		) {
			return false;
		} else if (
			name === 'eyr' &&
			!(parsedValue > 2020 && parsedValue <= 2030)
		) {
			return false;
		} else if (name === 'hgt') {
			const [, digits, unit] = value.match(/(\d{1,3})(cm|in)/u) ?? [];

			const height = parseInt(digits, 10);

			if (unit === 'cm' && !(height > 150 && height <= 193)) {
				return false;
			} else if (unit === 'in' && !(height > 59 && height <= 76)) {
				return false;
			}
		} else if (name === 'hcl' && !value.match(/#[0-9a-f]{6}/u)) {
			return false;
		} else if (
			name === 'ecl' &&
			!['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(value)
		) {
			return false;
		} else if (name === 'pid' && !value.match(/\d{9}/u)) {
			return false;
		}
	}

	return true;
};

export const firstStar = (input) => {
	let count = 0;

	for (const passport of input) {
		if (validatePassport(parsePassports(passport))) {
			count++;
		}
	}

	return count;
};

export const secondStar = (input) => {
	let count = 0;

	for (const passport of input) {
		if (validatePassport2(parsePassports(passport))) {
			count++;
		}
	}

	return count;
};
