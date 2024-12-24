"""
This one is too much data-munging to be enjoyable in Rust.
Run this, fixing things until it all passes, then run the results through a text diff.

gwh,jct,rcb,wbw,wgb,z09,z21,z39
"""
import re


with open("./input/day24") as f:
    data = f.read()

data = data.split("\n\n")[-1]


gates = data.split("\n")
parsed = []
for g in gates:
    if not g:
        continue
    (x0, op, x1, _, out) = g.split()
    parsed.append(({x0, x1}, op, out))
adder_parts = [[]]
carries = []

out = ({"x00", "y00"}, "XOR", "z00")
parsed.remove(out)
adder_parts[-1].append(out)
carry = next(g for g in parsed if g[0] == {"x00", "y00"} and g[1] == "AND")
adder_parts[-1].append(carry)
carries.append(carry[-1])
i = 1
for i in range(1, 45):
    print(i)
    adder_parts.append([])

    print("intermediate")
    ig = next(g for g in parsed if g[0] == {f"x{i:02}", f"y{i:02}"} and g[1] == "XOR")
    print(ig)
    adder_parts[-1].append(ig)
    intermediate = ig[-1]
    parsed.remove(ig)

    print("bits and")
    ba = next(g for g in parsed if g[0] == {f"x{i:02}", f"y{i:02}"} and g[1] == "AND")
    print(ba)
    adder_parts[-1].append(ba)
    parsed.remove(ba)
    ba = ba[-1]

    print("carry and")
    ca = next(g for g in parsed if g[0] == {carries[-1], intermediate} and g[1] == "AND")
    print(ca)
    adder_parts[-1].append(ca)
    parsed.remove(ca)
    ca = ca[-1]

    print("out")
    out = next(g for g in parsed if g[0] == {intermediate, carries[-1]} and g[1] == "XOR")
    print(out)
    adder_parts[-1].append(out)

    print("carry")
    carry = next(g for g in parsed if g[0] == {ba, ca} and g[1] == "OR")
    print(carry)
    adder_parts[-1].append(carry)
    parsed.remove(carry)
    carries.append(carry[-1])
