#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

class Base {
public:
	Base(int a);

};

class Child: public Base {
public:
	Child(int a, int b);

	void DoThing(int a);

	int b = 0;
};

#endif