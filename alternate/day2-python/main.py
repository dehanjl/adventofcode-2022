# the score the player gets when indexing by score_matrix[opponent][player]
score_matrix = [
    # player: rock, paper, scissors
    [3, 6, 0],  # opponent: A -> rock; draw, win, lose
    [0, 3, 6],  # opponent: B -> paper; lose, draw, win
    [6, 0, 3],  # opponent: C -> scissors; win, lose, draw
]


def read_file(filename: str) -> list[tuple[str, str]]:
    with open(filename, "r") as f:
        return [(line[0], line[2]) for line in f.readlines()]


def part1():
    score = 0
    for opponent, player in read_file("input.txt"):
        score += score_matrix["ABC".index(opponent)]["XYZ".index(player)] + ("XYZ".index(player) + 1)
    print("Part 1:", score)


def part2():
    score = 0
    for opponent, result in read_file("input.txt"):
        player_idx = score_matrix["ABC".index(opponent)].index("XYZ".index(result) * 3)
        score += score_matrix["ABC".index(opponent)][player_idx] + (player_idx + 1)
    print("Part 2:", score)


part1()
part2()
