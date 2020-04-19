#include "main.hpp"

int main() {
	int a = 32;

	bool test = a >= 0 && a <= -32;

	if(a >= 0 && a <= -32 && test) {
		print("worked");
	}
}