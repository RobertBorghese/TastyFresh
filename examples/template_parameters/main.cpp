#include "main.hpp"

int main() {

	std::vector<int> numberVec;

	for(int i = 0; i < 10; i++) {
		numberVec.push_back(i);
	}

	std::cout << "[[Vector info]]" << std::endl;
	std::cout << "SIZE:     " << numberVec.size() << std::endl;
	std::cout << "CAPACITY: " << numberVec.size() << std::endl;
	std::cout << "MAX SIZE: " << numberVec.max_size() << std::endl;
	std::cout << "FIRST:    " << numberVec[0] << std::endl;

	std::map<const char*, int> textToIntMap;

	textToIntMap["one"] = 1;
	textToIntMap["two"] = 2;
	textToIntMap["three"] = 3;

	convertStringToNumber("two", textToIntMap);
}

void convertStringToNumber(const char* str, std::map<const char*, int>& map) {
	std::cout << "The number \"" << str << "\" is " << map[str] << "." << std::cout;
}