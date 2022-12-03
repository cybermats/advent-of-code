import typing


def get_priority(item: str):
    if item.isupper():
        return ord(item) - ord('A') + 27
    return ord(item) - ord('a') + 1

def splitRucksack(contents: str):
    size = len(contents)
    mid = size // 2
    first = contents[:mid]
    second = contents[mid:]
    return first, second

def handle_line(line: str):
    first, second = splitRucksack(line)
    common = set(first).intersection(set(second))
    assert len(common) == 1
    c = common.pop()
    p = get_priority(c)
    return p


def main(filename):
    with open(filename) as f:
        sum = 0
        for line in f:
            sum += handle_line(line)
        print(sum)


if __name__ == "__main__":
    main("input.txt")