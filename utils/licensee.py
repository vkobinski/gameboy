names = []
names_rust = []
with open("./licensee.txt", "r") as file:
    for both in file:
        code, name = both.split("\t")
        name = name.replace("\n", "")
        names.append(name)

with open("./licensee_rust.txt", "r") as file_rust:
    for both in file:
        code, name = both.split("=")
        name = name.replace(" ", "")
        names_rust.append(name)

for i, name in enumerate(names):


