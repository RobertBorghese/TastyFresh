#include "examples/hello_world/main.hpp"

#include <iostream>

std::shared_ptr<vector> bla = nullptr;
vector bla;

int main() {
	std::cout << "Hello world" << std::endl;

	TestClass test = TestClass();
	test.printValue();

	test++;
	test++;
	test++;

	test.printValue();

	auto test = (int)(fdsf);

	auto test2 = static_cast<int>(32.0f);

	return 0;
}

int Test_Class::a = 0;

Test_Class::Test_Class(int a) {
	this->a = a;
}

Test_Class::Test_Class(float a) {
	this->a = (int)(a);
}

Test_Class::~Test_Class() { }

void Test_Class::printValue() {
	std::cout << "The value of 'a' is: " << a << std::endl;
}

void Test_Class::operator++() {
	a++;
}