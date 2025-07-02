with open("../data/d17.txt") as f:
    chunks = f.read().split("\n\n")

registers = [int(line.split(": ")[1]) for line in chunks[0].splitlines()]

ip = 0
program = [int(x) for x in chunks[1].split(": ")[1].split(",")]


def literal(x: int):
    return x


def combo(x: int):
    if x <= 3:
        return x
    return registers[x - 4]


def output(x: int):
    print(x)


def adv(x: int):
    registers[0] = registers[0] // 2 ** combo(x)


def bxl(x: int):
    registers[1] = registers[1] ^ literal(x)


def bst(x: int):
    registers[1] = combo(x) % 8


def jnz(x: int):
    global ip
    if registers[0] == 0:
        return

    ip = literal(x) - 2


def bxc(x: int):
    registers[1] = registers[1] ^ registers[2]


def out(x: int):
    return combo(x) % 8


def bdv(x: int):
    registers[1] = registers[0] // 2 ** combo(x)


def cdv(x: int):
    registers[2] = registers[0] // 2 ** combo(x)


instructions = [adv, bxl, bst, jnz, bxc, out, bdv, cdv]


def run_program() -> list[int]:
    global ip
    outputs = []
    while ip < len(program):
        opcode, x = program[ip], program[ip + 1]
        result = instructions[opcode](x)
        if result is not None:
            outputs.append(result)
        ip += 2
    return outputs


print("p1", ",".join(str(o) for o in run_program()))

m = 10000
while True:
    registers = [int(line.split(": ")[1]) for line in chunks[0].splitlines()]
    ip = 0
    registers[0] = m
    outputs = run_program()
    print(m, outputs)
    if outputs == program:
        break
    m += 1
