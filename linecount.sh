#!/bin/python
import pathlib

desktop = pathlib.Path("")
lines = 0
print("\033[92m", end="")
rust_files = []
shader_files = []
toml_files = []

IGNORED_DIRS = {"res", "target"}

for item in desktop.rglob("*"):
    if any(part in IGNORED_DIRS for part in item.parts):
        continue

    if item.is_file():
        extension = item.suffix.lstrip(".")

        if item.name == "Cargo.lock":
            continue

        if extension in ("wgsl", "rs", "wesl", "toml"):
            with open(item, 'r') as fp:
                linecount = len(fp.readlines())

            if extension == "rs":
                rust_files.append((item.name, linecount))
            elif extension in ("wgsl", "wesl"):
                shader_files.append((item.name, linecount))
            elif extension == "toml":
                toml_files.append((item.name, linecount))

            lines += linecount

rust_files.sort()
for (name, count) in rust_files:
    print(f"\t{name}::{count}")

print()
shader_files.sort()
for (name, count) in shader_files:
    print(f"\t{name}::{count}")

print()
toml_files.sort()
for (name, count) in toml_files:
    print(f"\t{name}::{count}")

print("\033[96mTOTAL:", lines)