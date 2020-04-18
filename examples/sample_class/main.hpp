#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

#include <iostream>

int main();

class Counter {
public:
	Counter();
	~Counter();

	int getCount();
	void increment();

	int count = 0;
};

#endif