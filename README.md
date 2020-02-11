# TastyFresh
Many C++ frameworks, such as Qt5 and Unreal Engine, are wonderful libraries that sit upon a tiresome language. While one would expect such frameworks to have a plethora of bindings for other, much palatable, system-level languages such as Rust or D, due to each frameworks' scale, complexity, and reliance on macros and other incompatible compile-time features, creating consistent, up-to-date bindings is an unfeasible task. Tasty Fresh is a programming language that aims to tackle this problem by transpiling directly to human-readable C++ without the need for explicit bindings. The language hopes to achieve this while also provding modern features and slicker syntax. Another way to look at Tasty Fresh is as a pseudo-superset/metaprogramming wrapper for C++.

By version `1.0.0`, Tasty Fresh hopes to include:

* No header files or archaic import systems, but features to help configure how they translate to C++ if necessary.
* Static-typing and null-safety for all Tasty Fresh code prior to being transpiled into C++.
* Allow for the usage of unknown classes and variables that may only exist in the C++ context.
* Have line numbers in the C++ source files match directly to the line numbers from the Tasty Fresh source files to help decypher errors and warnings that may arise from the C++ compiler.
* Modern, Rust-like enums with union storage and pattern matching.
* Static extensions for classes, primitives, and unknown C++ types.
* Smart `.` operators as opposed to explicit use of `->` or `::`.
* Basic type inference for variable initialization and function return types.
* Simple, yet powerful text-replacement meta-programming functions and Haxe-like abstract classes.
* Ability to directly inject C++ code in any location.
