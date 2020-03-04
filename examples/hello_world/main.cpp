#include "./examples/hello_world/main.hpp"

void main() {
	int bla1 = test(12);
	float bla2 = test("fdsfds");
}

int test(int a) {
	return 0;
}

float test(const char* a) {
	return 0.0f;
}