#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

class Base {
public:
	int a = 0;
};

class Child {
public:
	void test();

	Base* a = nullptr;
	Base b;
};

#endif