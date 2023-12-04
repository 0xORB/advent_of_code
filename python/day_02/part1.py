RED   = "red"
GREEN = "green"
BLUE  = "blue"

def main():
    with open("input.txt", 'r') as f:
        lines = f.readlines()
        lines = [line.replace("\n", "") for line in lines]
    f.close()
#     test = ("""
# Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
# Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
# Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
# Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
# Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green""").split("\n")
    # for t in test:
    #     print(t)
    # test_single = [test[3]]
    # output = part1(test_single)
    output = part1(lines)
    print(output)

def part1(input: list()) -> int:
    color_limits = {
        RED: 12,
        GREEN: 13,
        BLUE: 14
    }
    result = 0
    final_result = 0
    game_map = dict()
    sub_game_map = dict()
    ss = dict()
    dumb = dict()
    for line in input:
        game = line.split(":")
        game_map[game[0]] = game[1].split(";")

    for key, val in game_map.items():
        for idx, sub_game in enumerate(val):
            test = sub_game.split(",")
            ss = dict()
            for j in test:
                hello = j.strip().split(" ")
                ss[hello[1]] = int(hello[0])
            sub_game_map[idx] = ss
            # print(sub_game_map)
        val = sub_game_map
        print(f"{key} {val}")
        result = 0
        for skey, sval in val.items():
            score = 0
            for sskey, ssval in sval.items():
                if ssval <= color_limits.get(sskey):
                    score += 1
                    # print(f"score: {score}")
                # else:
                    # print(f"{skey}: sub game lost")
            # print("new sub game")
            if score == len(sval):
                result += 1
                # print(f"win game count: {result}")
        dumb[key] = result
        # print(dumb.get(key))
        # print(len(val))
        if dumb.get(key) == len(val):
            print("valid game")
            final_result += dumb.get(key)

    return final_result

if __name__ == '__main__':
    main()

# 0
# {'green': 2, 'blue': 12}
# 1
# {'red': 6, 'blue': 6}
# 2
# {'blue': 8, 'green': 5, 'red': 5}
# 3
# {'green': 5, 'blue': 13}
# 4
# {'green': 3, 'red': 7, 'blue': 10}
# 5
# {'blue': 13, 'red': 8}