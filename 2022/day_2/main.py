import sys

ROCK = 0
PAPER = 1
SCISSOR = 2

def winner(me, other):
    if me == other:
        return 0
    if me == ROCK and other == SCISSOR:
        return 1
    if me == SCISSOR and other == PAPER:
        return 1
    if me == PAPER and other == ROCK:
        return 1
    return -1

def convertOther(other):
    other = other.strip()
    if other == 'A':
        return ROCK
    if other == 'B':
        return PAPER
    if other == 'C':
        return SCISSOR
    raise RuntimeError("Unknown other type")


def convertOwn(me):
    me = me.strip()
    if me == 'X':
        return ROCK
    if me == 'Y':
        return PAPER
    if me == 'Z':
        return SCISSOR
    raise RuntimeError("Unknown me type: " + me)

def roundScore(me):
    return me + 1

def outcomeScore(me, other):
    outcome = winner(me, other)
    if outcome > 0:
        return 6
    if outcome == 0:
        return 3
    return 0

def totalScore(me, other):
    return roundScore(me) + outcomeScore(me, other)

def lineScore(line: str):
    items = line.split(" ")
    meSign = items[1]
    otherSign = items[0]
    me = convertOwn(meSign)
    other = convertOther(otherSign)
    return totalScore(me, other)



def main(file):
    with open(file, 'rt') as f:
        score = 0
        for line in f:
            score += lineScore(line)

    print(score)


if __name__ == "__main__":
    main("input.txt")