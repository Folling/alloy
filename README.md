# What
alloy is a collection of libraries necessary to create modern, flexible, and stylish applications.

It is made up of 6 different libraries:
- adhesive – Interaction with the underlying OS
- catalyst – Provision of utilities
- granite – Data persistence
- graphene – Abstraction of various rendering libraries
- graphite – GUI capabilities
- ink – Text & Font handling
- neon – 2D rendering

Together these form the perfect foundation one might need to programm their dream user-facing software.

# Why
Why indeed. I tried working with plenty of GUI libraries and frameworks in the past to create my own passion project 
and was left unsatisfied on all fronts.
Plenty of backwards compatibility related problems. A plethora of design decisions that simply do not make sense.
And worst of all: A lot of private interfaces that you cannot configure, etc.

And so graphite – a custom GUI framework was born and I was surprised just how well it worked.
graphite used to be the umbrella term for what is now alloy. In the current iteration graphite only describes the GUI capabilities.

It has existed in multiple forms before. Two C++ versions and one previous rust version. 
You can find these projects (in ascending order of the creation date):
- [Original C++ version](https://memleak.eu/Folling/graphite)
- [Original Rust version](https://memleak.eu/Folling/graphite-rs)
- [Second C++ iteration](https://memleak.eu/Folling/graphite-CPP-v2)

# How
You should look at my blog (link will follow soon) or the separate repositories and their subdirectories for further information.

# When
There is no fixed timeline for alloy. I have a fulltime job and will work on this at my own pace

# Contributing
alloy is open source and is supposed to benefit from the inclusion of other people. 
However I do reserve the rights to deny any feature requests or pull requests but am always open to discussion and having my mind changed. 
If you're uncertain whether or not a certain pull request would be appreciated and don't want to waste effort without knowing whether it's worth it, feel free to open an issue and ask. 
All code should be formatted using the same guideline. For this please use rustfmt. In the future a customised rustfmt stylisation might be used.
File and directory names are are to be formatted using snake_case. Excluded from this rule are files that have a certain convention such as .gitignore, LICENCE.txt and markdown files.

# Support
I have a fulltime job and can only afford so much time for alloy (the root project of graphite). If you would like to change that in the future consider donating to the project (note: Donating link will follow, graphite isn't worth donating yet). I also appreciate feedback (next to constructive criticism) so feel free to email me at coding@folling.de. 
