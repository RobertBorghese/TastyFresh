<p align="center">
  <a><img src="https://i.imgur.com/lD7bEE9.png" /></a>
</p>
<p align="center">
	<a href="https://github.com/RobertBorghese/TastyFresh/actions?query=workflow%3ARust"><img src="https://github.com/RobertBorghese/TastyFresh/workflows/Rust/badge.svg" /></a>
	<a href="https://github.com/RobertBorghese/TastyFresh/blob/master/LICENSE"><img src="https://img.shields.io/github/license/RobertBorghese/TastyFresh" alt="License" /></a>
	<a href="https://discord.gg/2RTssnA"><img src="https://img.shields.io/discord/701041775267020820.svg?logo=discord" alt="Discord" /></a>
</p>

---

C++ frameworks like Qt5 and Unreal Engine are wonderful libraries that unfortunately rest upon a tiresome language. One would expect such frameworks to have a plethora of bindings for other, palatable, system-level languages such as Rust or D. However, due to each frameworks' scale, complexity, and reliance on macros/unique compile-time configurations, creating consistent, up-to-date bindings is an unfeasible task for most of them.

Tasty Fresh is a programming language that aims to tackle this problem by transpiling directly to human-readable C++ without the need for explicit bindings. The language hopes to achieve this while also provding modern features and slicker syntax. Another way to look at Tasty Fresh is as a pseudo-superset/metaprogramming wrapper for C++.

---

With that being said, it is very unlikely Tasty Fresh will ever be production ready. It is a small, *MESSY*, one-man project aimed solely at the needs of said main. Bugs, issues, and other unintended behavior will be fixed as needed, and as a result, one must have a relatively advanced understanding of both C++ and Tasty Fresh to receive any benefit from the language. Nonetheless, contributions to clean-up the code base or elevate this language to production quality are always welcome. 

Ultimately, if Tasty Fresh ever reaches version `1.0.0`, it should include:

* No header files or archaic import systems, but features to help configure how they translate to C++ if necessary. (✔️)
* Static-typing and null-safety for all Tasty Fresh code prior to being transpiled into C++.
* Allow for the usage of unknown classes and variables that may only exist in the C++ context. (✔️)
* Have C++ source line numbers match the line numbers from the Tasty Fresh source to help decypher C++ errors and warnings. (✔️)
* Modern, Rust-like enums with union storage and pattern matching.
* Static extensions for classes, primitives, and unknown C++ types.
* Smart `.` operators as opposed to explicit use of `->` or `::`. (✔️)
* Basic type inference for variable initialization and function return types.
* Simple, yet powerful text-replacement meta-programming functions and Haxe-like abstract classes.
* Ability to directly inject C++ code in any location.

---

# Examples

## Basic Output

*main.tasty*
```rust
include system iostream;

fn main() {
	std.cout << "I am a depression." << std.endl;
}
```

*main.cpp (output)*

```cpp
#include "main.hpp"

#include <iostream>

void main() {
	std::cout << "I am a depression." << std::endl;
}
```

## Memory Management

*main.tasty*
```rust
include system iostream;

fn main() {
	copy myNumber = 100;
	ref myNumberRef = myNumber;
	ptr myNumberPtr = myNumber;
	
	myNumber++;
	myNumberRef++;
	myNumberPtr++;
	
	std.cout << myNumber << std.endl;
}
```

*main.cpp (output)*

```cpp
#include "main.hpp"

#include <iostream>

void main() {
	int myNumber = 100;
	int& myNumberRef = myNumber;
	int* myNumberPtr = &myNumber;
	
	myNumber++;
	myNumberRef++;
	(*myNumberPtr)++;
	
	std::cout << myNumber << std::endl;
}
```


## Tuples

*main.tasty*
```rust
include system iostream;

fn main() {
	copy myTuple = (100, 200, 300);
	printTuple(myTuple);
}

fn printTuple(tuple: (int, int, int)) {
	std.cout << tuple.0 << tuple.1 << tuple.2 << std.endl;
}
```

*main.cpp (output)*

```cpp
#include "main.hpp" // "#include <tuple>" automatically added within header file.

#include <iostream>

void main() {
	std::tuple<int, int, int> myTuple = std::make_tuple(100, 200, 300);
	printTuple(myTuple);
}

void printTuple(std::tuple<int, int, int> tuple) {
	std::cout << std::get<0>(tuple) << std::get<1>(tuple) << std::get<2>(tuple) << std::endl;
}
```


## "New" Inference

*main.tasty*
```rust
class Test {}

fn main() {
	let test = new Test();

	copy test2 = new Test();

	ptr test3 = new Test();
	delete test3;

	autoptr test4 = new Test();

	uniqueptr test5 = new Test();
}
```

*main.cpp (output)*

```cpp
#include "main.hpp"

void main() {
	Test test;

	Test test2;

	Test* test3 = new Test();
	delete test3;

	std::shared_ptr<Test> test4 = std::make_shared<Test>();

	std::unique_ptr<Test> test5 = std::make_unique<Test>();
}
```


## Templates

*main.tasty*
```rust
include system iostream;
include system vector;

fn main() {
	let intList = new std.vector@int();
	for i in 0..10 {
		intList.push_back(i);
	}
	for num in intList {
		std.cout << num << std.endl;
	}
}
```

*main.cpp (output)*

```cpp
#include "main.hpp"

#include <iostream>
#include <vector>

void main() {
	std::vector<int> intList;
	for(int i = 0; i < 10; i++) {
		intList.push_back(i);
	}
	for(auto num : intList) {
		std::cout << num << std::endl;
	}
}
```
