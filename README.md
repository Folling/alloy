# Summary
alloy is a collection of libraries necessary to create modern, flexible, and stylish applications.

It is made up of 6 different libraries:
| Name            | Link                                  | Description                                    |
|-----------------|---------------------------------------|------------------------------------------------|
| adhesive        | https://github.com/Folling/adhesive   | Interaction with the underlying OS             |
| catalyst        | https://github.com/Folling/catalyst   | Provision of utilities                         |
| granite         | https://github.com/Folling/granite    | Data persistence                               |
| graphene        | https://github.com/Folling/graphene   | Abstraction of various rendering libraries     |
| graphite        | https://github.com/Folling/graphite   | GUI capabilities                               |
| ink             | https://github.com/Folling/ink        | Text & Font handling                           |
| neon            | https://github.com/Folling/neon       | 2D rendering                                   |

Together these form a sophisticated foundation for all your application development needs.

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
There is no fixed timeline for alloy. I have a fulltime job and will work on this at my own pace.

# Focus
I attempt to use as few dependencies as possible. 
The reasons for this decision are plentiful and go beyond the scope of this README.
Here is an overview of the types of dependencies I will permit, all of these will use the system native C library:
1. Systems libraries
2. Security Related Libraries

And here is an overview of the libraries that will be used to achieve this:
#### Linux
- Xlib (see [adhesive](https://github.com/Folling/adhesive) on an explanation why I'm not using XCB)
- Wayland
- GLX
- FontConfig
- FreeType
- HarfBuzz
- Uuid

#### Windows
- WinApi
- WGL
- TBD

#### MacOS
- Cocoa
- EGL
- TBD

Everything else will be written from scratch.

One thing I found bothersome when working with with other libraries were private sections of the API.
Whilst private APIs can be useful to avoid users accidentally messing up or forming a dependency on a 
volatile part not having the option to access them enforces a certain rigidity that can be hard to work with 
if you ever need to perform tasks outsides of the authors' assumptions.
For example, the tooltip delay was not changeable in previous JavaFX versions. 
The field existed, but you simply couldn't access it.

To workaround this design "flaw" the APIs within alloy will follow a system similar to "pimpl":

Every struct has an "inner" field with all fields that are supposed to be private and an 
unsafe getter function to access it. This forces people to opt-in with the concious decision and understanding 
that what they are doing is inherently prone to break if the API changes in the future.

# Contributing
alloy is open source and is supposed to benefit from the inclusion of different people with different ideas. 
At least at the beginning however, this project will be managed top-down. I reserve all rights to deny feature requests,
close tickets, ignore recommendations, the whole authoritarian software owner deal.
This might change in the feature if the project builds a community that is large enough.
If you're uncertain whether or not a certain pull request would be appreciated and 
don't want to waste effort without knowing whether it's worth it, feel free to open an issue and ask.

All code should be formatted using the same guideline. For this please use rustfmt. 
In the future a customised rustfmt stylisation might be used.
File and directory names are are to be formatted using `nocase`. 
Excluded from this rule are files that have a certain convention such as:
`.gitignore`, `Cargo.toml`, `LICENCE.txt`, or the `README.md`.

# Support
I have a fulltime job and can only afford so much time for alloy.
This will probably also not change in the forseeable future, even if donations could cover my basic living expenses.
That being said I do take donations, both as a way to say "thanks" and as a way to help build up the project.
(note: Donating link will follow, alloy isn't worth donating yet). 
I also appreciate feedback (next to constructive criticism) so feel free to email me at coding@folling.de. 

# Naming
An alloy is an admixture of metals, or a metal combined with one or more other elements. 
I chose chemistry/geology related terms for the sub-libraries, so alloy is a combination of the many elements.
