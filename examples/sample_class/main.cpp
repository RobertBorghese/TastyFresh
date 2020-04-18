#include "main.hpp"

Counter::Counter() {
	std::cout << "Created Counter!" << std::endl;
}

Counter::~Counter() {
	std::cout << "Destroyed Counter!" << std::endl;
}

int Counter::getCount() {
	return count;
}

void Counter::increment() {
	count++;
}

int main() {
	Counter c;

	std::cout << c.getCount() << std::endl;

	for(int i = 0; i < 10; i++) {
		c.increment();
	}

	std::cout << c.getCount() << std::endl;
}