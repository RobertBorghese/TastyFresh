#include "main.hpp"

char string_second(const std::string& self) {
	return self.size() >= 2 ? self.at(1) : 0;
}

int main() {
	std::string str("123456");

	size_t size = str.size();
	char second_char = string_second(str);

	std::cout << "Size is: " << size << std::endl;
	std::cout << "Second char is: " << second_char << std::endl;
}