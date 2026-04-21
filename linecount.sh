#!/bin/python
import pathlib
desktop = pathlib.Path("src")

desktop.iterdir()

lines = 0
print("\033[92m",end="")

for item in desktop.rglob("*"):
    if item.is_file():
        if item.name.split(".")[1] == "wgsl" or item.name.split(".")[1] == "rs":
            with open(item, 'r') as fp:
                linecount = len(fp.readlines())
            print("\t"+str(item).split("/")[-1]+"::"+str(linecount))
            lines += linecount

print("\033[96mTOTAL:",lines)