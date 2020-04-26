#include "main.hpp"

#include "alt.hpp"

int main() {
	std::string b;
	size_t c = b.size();
	size_t d = string_test(b);

	std::cout << c << d << std::endl;
}