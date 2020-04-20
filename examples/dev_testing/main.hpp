#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

int main();

class Test {
public:
	int a = 32;
};

class Test2 {
public:
	Test b = Test();
};

#endif