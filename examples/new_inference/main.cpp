#include "main.hpp"

int main() {

	std::list classList1;
	std::list classList2(2, 4);

	std::list* classListPtr = new std::list(2, 4);
	delete classListPtr;

	std::shared_ptr<std::shared_ptr<std::list>> classListAuto = std::make_shared<std::list>(2, 4);

	std::unique_ptr<std::unique_ptr<std::list>> classListUnique = std::make_unique<std::list>(2, 4);
}