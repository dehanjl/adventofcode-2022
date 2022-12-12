def prio(c: str) -> int:
    if c.isupper():
        return ord(c) - 38
    else:
        return ord(c) - 96


def read_file_1(filename: str) -> list[tuple[list[int], list[int]]]:
    l = []
    with open(filename, "r") as f:
        for line in f.readlines():
            ll = [prio(c) for c in line.strip()]
            l.append((ll[: (len(ll) // 2)], ll[(len(ll) // 2) :]))
    return l


def read_file_2(filename: str) -> list[tuple[list[int], list[int], list[int]]]:
    l = []
    with open(filename, "r") as f:
        lines = f.readlines()
        for i in range(0, len(lines), 3):
            for idx, g in enumerate(lines[i : i + 3]):
                lines[i + idx] = [prio(_) for _ in g]
            l.append((lines[i], lines[i + 1], lines[i + 2]))

    return l


def part1():
    data = read_file_1("input.txt")
    sum = 0
    for x, y in data:
        for i in x:
            if i in y:
                sum += i
                break

    print("Part 1: ", sum)


def part2():
    data = read_file_2("input.txt")
    sum = 0
    for x, y, z in data:
        for i in x:
            if i in y and i in z:
                sum += i
                break
    print("Part 2: ", sum)


part1()
part2()
