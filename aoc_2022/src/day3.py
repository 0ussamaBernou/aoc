def readline(filename: str):
    with open(filename, "r") as f:
        for line in f:
            yield line


def part_1(filename: str):
    type = ""
    priorities = 0
    for line in readline(filename):
        # half = int(len(line) / 2)
        half = len(line) // 2
        left = line[:half]
        right = line[half:]
        for char in left:
            if char in right:
                type = char
                break
        if type.isupper():
            priority = ord(type) - 38
        elif type.islower():
            priority = ord(type) - 96
        priorities += priority
    return priorities


if __name__ == "__main__":
    filename = "day3.input"
    priorities = part_1(filename)
    print(f"priorities: {priorities}")
