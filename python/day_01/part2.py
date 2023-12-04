def main():
    with open("input.txt", 'r') as f:
        lines = f.readlines()[:5]
        lines = [line.replace("\n", "") for line in lines]
    f.close()

    # output = part1(lines)
    output = part2(lines)
    print(output)


def part1(input: list()) -> int:
    result = list()
    for line in input:
        nums = ""
        for ch in line:
            if ch.isnumeric():
                nums += ch
        result.append(int(f"{nums[0]}{nums[-1]}"))

    return sum(result)

def part2(input: list()) -> int:
    num_map = {
        "one": "o1e",
        "two": "t2o",
        "three": "t3e",
        "four": "f4r",
        "five": "f5e",
        "six": "s6x",
        "seven": "s7n",
        "eight": "e8t",
        "nine": "n9e",
    }
    result = list()
    for line in input:
        for name, value in num_map.items():
            line = line.replace(name, value)
        nums = ""
        for ch in line:
            if ch.isnumeric():
                nums += ch
        result.append(int(f"{nums[0]}{nums[-1]}"))
    return sum(result)

if __name__ == '__main__':
    main()