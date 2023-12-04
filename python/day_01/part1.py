def main():
    with open("input.txt", 'r') as f:
        lines = f.readlines()
        lines = [line.replace("\n", "") for line in lines]
    f.close()

    output = part1(lines)
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

if __name__ == '__main__':
    main()