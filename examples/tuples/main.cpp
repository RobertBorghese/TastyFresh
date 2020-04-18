#include "main.hpp"

void printMyTuple(std::tuple<int, const char*>& tuple) {
	std::cout << "Number is: " << tuple.0 << std::endl;
	std::cout << "String is: " << tuple.1 << std::endl;
}

int main() {
	std::tuple<int, const char*> myTuple = std::make_tuple(12, "Blabla");
	printMyTuple(myTuple);
}