#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

#include <memory>

extern std::shared_ptr<vector> bla;
extern vector bla;

int main();

class Test_Class {
public:
	Test_Class(int a);
	Test_Class(float a);
	virtual ~Test_Class();
	void printValue();
	void operator++();

	static int a;
	vector<int> b = vector<int>();
	vector<int>* ghj = 0;
};

#endif