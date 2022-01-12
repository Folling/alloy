# Summary
alloy is a collection of libraries necessary to create modern, flexible, and stylish applications.

It is made up of 6 different libraries:
- [adhesive](https://github.com/Folling/adhesive) – Interaction with the underlying OS
- [catalyst](https://github.com/Folling/catalyst) – Provision of utilities
- [granite](https://github.com/Folling/granite) – Data persistence
- [graphene](https://github.com/Folling/graphene) – Abstraction of various rendering libraries
- [graphite](https://github.com/Folling/graphite) – GUI capabilities
- [ink](https://github.com/Folling/ink) – Text & Font handling
- [neon](https://github.com/Folling/neon) – 2D rendering

Together these form a sophisticated foundation one might need to program their dream user-facing software.

# Development
If you want to build/contribute to alloy, please note that a few requirements must be met:
1. You must be using a unix-based system. If you are developing on windows, please use the WSL.
2. pkgconf (note, not pkg-config) must be installed on the system.

# Motivation
I tried working with plenty of GUI libraries and frameworks in the past to create my own passion project 
and was left unsatisfied on a lot of fronts.
Plenty of backwards compatibility related problems. A plethora of design decisions that simply do not make sense.
And worst of all: A lot of private interfaces that you cannot configure.

And so graphite – a custom GUI framework was born and I was surprised just how well it worked.
graphite used to be the umbrella term for what is now alloy. In the current iteration graphite solely describes the GUI capabilities.

It has existed in multiple forms before. Two C++ versions and one previous rust version. 
You can find these projects (in ascending order of the creation date):
- [Original C++ version](https://memleak.eu/Folling/graphite)
- [Original Rust version](https://memleak.eu/Folling/graphite-rs)
- [Second C++ iteration](https://memleak.eu/Folling/graphite-CPP-v2)

# How
You should look at my blog (link will follow soon) or the separate repositories and their subdirectories for further information.

# When
There is no fixed timeline for alloy. I have a fulltime job and will work on this at my own pace

# Focus
I attempt to use as few dependencies as possible. There are however a few crates that I consider trivial and will not 
count toward this ideal. 
The reasoning here is that they are included in a LOT of projects and are very mature and secure. If any of them fail or are compromised
I'd have a much bigger problem than just alloy.
These are currently:
- [bitflags](https://crates.io/crates/bitflags)
- [byteorder](https://crates.io/crates/byteorder)
- [chrono](https://crates.io/crates/chrono)
- [lazy_static](https://crates.io/crates/lazy_static)
- [libc](https://crates.io/crates/libc)
- [log](https://crates.io/crates/log)
- [num-traits and all related crates](https://crates.io/crates/num-traits)
- [proc-macro2](https://crates.io/crates/proc-macro2)
- [quote](https://crates.io/crates/quote)
- [rand](https://crates.io/crates/rand)
- [regex](https://crates.io/crates/regex)
- [serde](https://crates.io/crates/serde)
- [syn](https://crates.io/crates/syn)
- [time](https://crates.io/crates/time)

Additionally there are a few crates which I consider vital to development with rust and will justify here once instead of
repeating the justification in every subproject:
- [bindgen](https://crates.io/crates/bindgen) | Necessary for useful interaction with underlying C APIs, such as the winapi, Xlib, or OpenGL
- [itertools](https://crates.io/crates/itertools) | Provides a lot of helpers that make writing certain code a lot cleaner and easier, although it could be considered to write a selfmade version as part of [catalyst](https://github.com/Folling/catalyst)
- [thiserror](https://crates.io/crates/thiserror) | An incredible library that makes error handling in rust what it should be.
- [strum](https://crates.io/crates/strum) | Wonderful and lightweight helpers to make working with enums a lot easier

All other dependencies will have a written justification in their respective project.

One thing I found bothersome when working with other APIs were private APIs.
Private APIs can be useful to avoid users accidentally messing something up, or forming a dependency on a volatile part of your API.
But not even having the option to access these parts makes certain aspects far too rigid.
For example, the tooltip delay was not changeable in previous JavaFX versions. The field existed, but you simply couldn't access it.
For this reason, the APIs within alloy will follow a system similar to "pimpl". Every struct has an "inner" field with all fields that are supposed to be private and an unsafe getter function to access it. This forces people to opt-in with the concious decision and understanding that what they are doing is inherently prone to break if the API changes in the future.

# Contributing
alloy is open source and is supposed to benefit from the inclusion of other people. 
However I do reserve the rights to deny any feature requests or pull requests but am always open to discussion and having my mind changed. 
If you're uncertain whether or not a certain pull request would be appreciated and don't want to waste effort without knowing whether it's worth it, feel free to open an issue and ask. 
All code should be formatted using the same guideline. For this please use rustfmt. In the future a customised rustfmt stylisation might be used.
File and directory names are are to be formatted using snake_case. Excluded from this rule are files that have a certain convention such as .gitignore, LICENCE.txt and markdown files.

# Support
I have a fulltime job and can only afford so much time for alloy. If you would like to change that in the future consider donating to the project (note: Donating link will follow, alloy isn't worth donating yet). I also appreciate feedback (next to constructive criticism) so feel free to email me at coding@folling.de. 

# Naming
An alloy is an admixture of metals, or a metal combined with one or more other elements. 
- Wikipedia
I chose chemistry/geology related terms for the sub-libraries, so alloy is a combination of the many elements.
