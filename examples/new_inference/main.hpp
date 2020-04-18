#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

#include <memory>
#include <list>

int main();

class MyClass {
public:
	std::list classList1 = std::list();
	std::list classList2 = std::list(2, 4);
};

#endif