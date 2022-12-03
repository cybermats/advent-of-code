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
    if other == 'A':
        return ROCK
    if other == 'B':
        return PAPER
    if other == 'C':
        return SCISSOR
    raise RuntimeError("Unknown other type")


def convertOwn(me_sign, other):
    if me_sign == 'X': # I need to lose
        if other == ROCK:
            return SCISSOR
        if other == PAPER:
            return ROCK
        if other == SCISSOR:
            return PAPER
    if me_sign == 'Y': # I need to draw
        return other
    if me_sign == 'Z': # I need to win
        if other == ROCK:
            return PAPER
        if other == PAPER:
            return SCISSOR
        if other == SCISSOR:
            return ROCK
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
    me_sign = items[1].strip()
    other_sign = items[0].strip()
    other = convertOther(other_sign)
    me = convertOwn(me_sign, other)
    return totalScore(me, other)



def main(file):
    with open(file, 'rt') as f:
        score = 0
        for line in f:
            score += lineScore(line)

    print(score)


if __name__ == "__main__":
    main("input.txt")