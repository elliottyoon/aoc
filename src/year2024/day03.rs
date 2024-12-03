type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let (mut part1, mut part2) = (0, 0);
    let mut is_enabled = true;

    let mut index = 0;
    let input = input.as_bytes();

    while index < input.len() {
        // We only care about "mul(x,y)", "do()" and "don't()" tokens.
        // Everything else is garbage.
        if input[index] != b'm' && input[index] != b'd' {
            index += 1;
            continue;
        }

        if input[index..].starts_with(b"mul(") {
            index += 4;

            // first number
            let mut first = 0;
            while input[index].is_ascii_digit() {
                first = 10 * first + (input[index] - b'0') as u32;
                index += 1;
            }

            // ',' delimeter
            if input[index] != b',' {
                continue;
            }
            index += 1;

            // second number
            let mut second = 0;
            while input[index].is_ascii_digit() {
                second = 10 * second + (input[index] - b'0') as u32;
                index += 1;
            }

            // ')' delimeter
            if input[index] != b')' {
                continue;
            }
            index += 1;

            let product = first * second;
            part1 += product;
            if is_enabled {
                part2 += product
            }
        } else if input[index..].starts_with(b"do()") {
            index += 4;
            is_enabled = true;
        } else if input[index..].starts_with(b"don't()") {
            index += 7;
            is_enabled = false;
        } else {
            index += 1;
        }
    }

    (part1, part2)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
