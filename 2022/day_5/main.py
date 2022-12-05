import typing
from collections import deque


class Command:
    def __init__(self, line: str):
        self.count = 0
        self.source = 0
        self.destination = 0
        self._parse(line)

    def _parse(self, line: str):
        items = line.split(" ")
        self._parse_command(items[0], items[1])
        self._parse_command(items[2], items[3])
        self._parse_command(items[4], items[5])

    def _parse_command(self, cmd: str, arg: str):
        if cmd == 'move':
            self.count = int(arg)
        elif cmd == 'from':
            self.source = int(arg) - 1
        elif cmd == 'to':
            self.destination = int(arg) - 1

class Stack:
    def __init__(self):
        self.stack = []

    def parse(self, lines: typing.List[str]):
        for line in lines:
            count = len(line) // 4
            while len(self.stack) < count:
                self.stack.append(deque())

            for i in range(count):
                item = line[i * 4 + 1: i * 4 + 2]
                if item == ' ':
                    continue
                self.stack[i].appendleft(item)

    def execute(self, cmd: Command):
        for c in range(cmd.count):
            item = self.stack[cmd.source].pop()
            self.stack[cmd.destination].append(item)

    def top(self):
        return [i[-1] for i in self.stack]

def main(filename):
    with open(filename) as f:
        lines = []
        for line in f:
            if not line or line.isspace():
                break
            lines.append(line)
        stack = Stack()
        stack.parse(lines)
        for line in f:
            cmd = Command(line)
            stack.execute(cmd)
        print(stack.top())


if __name__ == "__main__":
    main("input.txt")