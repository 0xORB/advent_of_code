def main():
    with open("input.txt") as f:
        lines = f.read().splitlines()
    output = part1(lines)
    print(output)

RED   = "red"
GREEN = "green"
BLUE  = "blue"

RED_MAX   = 12
GREEN_MAX = 13
BLUE_MAX  = 14

MAX_AMOUNTS = {RED: RED_MAX, GREEN: GREEN_MAX, BLUE: BLUE_MAX}

def part1(input: list()) -> int:
    result = 0
    for line in input:
        game, cubes_all = line.split(":")
        cube_groups = cubes_all.split(";")

        game_possibilities = list()
        for group in cube_groups:
            amounts = {"red": 0, "green": 0, "blue": 0}
            cubes = group.split(",")
            for cube in cubes:
                amount, color = cube.strip().split()
                amounts[color] += int(amount)

            game_possibilities.append(
                all((MAX_AMOUNTS[color] >= amount for color, amount in amounts.items()))
            )
            # [print(color, amount) for color, amount in amounts.items()]

        if all(game_possibilities):
            result += int(game.split()[-1])
    return result

if __name__ == '__main__':
    main()