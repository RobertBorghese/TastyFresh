#include "main.hpp"

int main() {

	int myVar = 0;

	if(myVar == 10) {
		myVar += 10;
	} else if(myVar == 0) {
		myVar++;
	} else {
		myVar -= 10;
	}

	for(int i = 0; i < 100; i++) {
		myVar += i;
	}

	for(int i = 99; i > 0; i--) {
		myVar -= i;
	}

	while(myVar < 110) {
		myVar += 2;
	}

	do {
		myVar -= 5;
	} while(myVar > 80);

	std::cout << myVar << std::endl;
}