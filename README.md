# Recursive `pathchk`

> [!IMPORTANT]
> This program doesn't even compile, yet.
> I've published this for backup purposes,
> and to allow people to contribute while I work on this

The default behavior (no args) is equivalent to
```sh
find . -printf '%f\0' | xargs -0r pathchk -pP
```
because this
```sh
find . -print0 | xargs -0r pathchk -pP
```
checks all components, but we only want to check `basename`s

The [example shown in the POSIX docs](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/pathchk.html#tag_20_93_17):
```sh
find . -exec pathchk -p -P {} +
```
is slower, because it doesn't batch args

## Why
[This post by David Wheeler](https://dwheeler.com/essays/fixing-unix-linux-filenames) was the "spark" that motivated me to make this. I've read somewhere that he wrote a sanitization program, but I couldn't find it.

The purpose of `rpck` is to ease the transition to stricter path-name rules, by finding all the paths that may need renaming.

I'm considering to add a configurable auto-fix feature, but that may be too complicated

## Usage

### Install
This needs a Rust toolchain. Recommended command:
```sh
cargo install --path . --config 'build.rustflags="-C target-cpu=native"'
```
Assuming you've downloaded and `cd`ed into the repo

### Run
Invoke the program by passing the paths you want to check:
```sh
rpck file.txt directory/
```
Or simply pass nothing, identical to `rpck .`.

The program will validate the args you pass, as `pathchk -pP` would. Then, for each directory, it will recursively check all `basename`s.

> [!note]
> I'm considering to print a warning when an arg ends with `/` but it doesn't exist in the file-system.

You can pass flags to change the rules. The one I recommend the most is `-l` (length), which allows bypassing the overly-pedantic 14char limit.

## etc
This program is single-threaded, as it's IO-bound.

## See also
- [This crate](https://github.com/xpe/posix-portable-filename). I didn't `use` it here, because I need more control over the rules
