#include "./examples/hello_world/main.hpp"

#include <iostream>

int glob1 = 65;
int glob2 = 32;

void main()  { 
	std::cout << "Hello, World!" << std::endl;
	std::cout << "Secnd line combo" << std::endl;

	long double a = 31.2l;
	std::cout << "The value of \"a\" is: " << a << "!" << std::endl;
	std::cout << "The value of \"glob\" is: " << glob << "!" << std::endl;

	const char*& b = "Putting char array in variable \"b\"";
	b.test = 32;
	std::tuple<const char*, int> fds = std::make_tuple("jfdklsfjds", 54);

	std::shared_ptr<int> fds = nullptr;
	vector<std::tuple<int**, int>> bla;
	int*** bla = 32;

	return 123;
}

std::tuple<unsigned int, long double>* secondFunction()  { 
	return std::make_tuple(43, 64);
}