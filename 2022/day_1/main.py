import sys

def handle(values, value):
    values.append(value)
    values.sort()
    while len(values) > 3:
        values.pop(0)


def main(file):
    with open(file, 'rt') as f:
        values = []
        sum_value = 0
        for line in f:
            if line and not line.isspace():
                sum_value += int(line)
            else:
                handle(values, sum_value)
                sum_value = 0
        handle(values, sum_value)
        print(values)
        print(sum(values))

if __name__ == "__main__":
    main(sys.argv[1])
