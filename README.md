![cargo test](https://github.com/curatorsigma/critic/actions/workflows/rust.yml/badge.svg)

# critic
`critic` is a language agnostic toolchain for computational textual criticism.

# What can critic do?
TODO

# Getting started
TODO

# Structure of the source code
- `critic` builds uppon [`critic-core`](https://github.com/curatorsigma/critic-core), which defines low-level language agnostic functions like ATG parsing.
- All aspects of individual languages are defined in `dialect`. The languages you want can be added by compiling `critic` with the appropriate cargo features.

TODO: other parts of the source

# License
This project is licensed under MIT-0 (MIT No Attribution).
By contributing to this repositry, you agree that your code will be licensed as MIT-0.

For my rationale for using MIT-0 instead of another more common license, please see
https://copy.church/objections/attribution/#why-not-require-attribution .

