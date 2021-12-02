commands = open("input").read().split("\n")

depth = 0
horizont = 0
aim = 0
for command in commands:
    if command == "":
        continue
    x = command.split(" ")
    verb = x[0]
    val = x[1]
    if verb == "forward":
        horizont += int(val)
        depth += (aim * int(val))
    if verb == "down":
        aim += int(val)
    if verb == "up":
        aim -= int(val)

print(depth*horizont)
