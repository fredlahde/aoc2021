commands = open("input").read().split("\n")

depth = 0
horizont = 0
for command in commands:
    if command == "":
        continue
    x = command.split(" ")
    verb = x[0]
    val = x[1]
    if verb == "forward":
        horizont += int(val)
    if verb == "down":
        depth += int(val)
    if verb == "up":
        depth -= int(val)

print(depth*horizont)
