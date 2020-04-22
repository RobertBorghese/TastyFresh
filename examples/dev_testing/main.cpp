#include "main.hpp"

int main() {
	int a = 32;
	test([=]() {
		std::cout << a << std::endl;
	});

	long long test = 32;
}

void test(std::function<long long()> a) {
	a();
	a();
}