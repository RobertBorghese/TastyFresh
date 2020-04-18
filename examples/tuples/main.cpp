#include "main.hpp"

void printMyTuple(const std::tuple<int, const char*>& tuple) {
	std::cout << "Number is: " << std::get<0>(tuple) << std::endl;
	std::cout << "String is: " << std::get<1>(tuple) << std::endl;
}

int main() {
	std::tuple<int, const char*> myTuple = std::make_tuple(12, "Blabla");
	printMyTuple(myTuple);
}