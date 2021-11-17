# Lazerbeak12345's Minimal Shell

A shell that seeks to only provide the bare minimum of utilities needed to be `/bin/sh` compatible, written in rust, with a goal of WASM+WASI compatibility. It's made for a specific project that I haven't yet made public.

## Name

LMSH stands for **L**azerbeak12345's **M**inimal **Sh**ell.

In some cases I've use Micro instead of Minimal. Those cases were typos.

## Checklist

- [ ] It meets the specification defined at [the 2008 UNIX standard here](https://www.ee.ryerson.ca/~courses/ele709/susv4/utilities/V3_title.html)
	- [ ] This will be running with the assumption that any commands often used with `/bin/sh` are provided, and that this software will only provide the bare minimum built-in commands [(such as `cd`)](https://unix.stackexchange.com/questions/38808/why-is-cd-not-a-program). These built-in commands are:
		- [ ] `cd`
		- [ ] `break`
		- [ ] `continue`
		- [ ] `exec`
	- [ ] It must have the correct exit codes.
	- [ ] It must correctly handle `/bin/sh` arguments.
- [ ] It can correctly execute my `/etc/profile` (default for Fedora 34) (this includes the several scripts in `/etc/profile.d/`)

Features not in the UNIX standard for `/bin/sh` will only be considered if

- This shell already meets the standard. Meeting the standard is priority 1.
- This feature will not be available if this shell is called as `/bin/sh`.
- It will not change the behavior of this shell when in `/bin/sh` mode.
- It must be optional at compile time (you should be able to choose to not compile bash support, for example).

In the event that a newer standard than the 2008 UNIX standard is released, unless meeting that standard is significantly easier than this one, this standard must be met first.

## Versioning

I'll be releasing version 1.0.0 only when I have reached 100% code coverage and when I've reached 100% of the checklist.

This package will be uploaded to cargo, and I'll start incrementing the version number once I reach 80% of the boxes on the checklist.

## Future goals

These are ideas that **might** be supported in the future, but only after we achieve the checklist.

- Macro support (ex you can make a macro to overload the behavior of $ to include more features or perhaps include more statements)
- bash support
- zsh support

## Alternatives

- Is 100% UNIX/POSIX compatibility not what you need? [Here's a different rust project called "oursh"](https://crates.io/crates/oursh) that is similar, but has a different priority list ("Fancy features should not be prevented by POSIX compatibility. This will affect the design of the shell").
- Do you need a few non-compliant bashisms in sh mode, at the cost of preformance and wasm support? Take a look at dash.

## License

The license that this falls under is the MIT license

Furthermore, on any earlier version of this software, you may choose to use it under the terms of the MIT.

