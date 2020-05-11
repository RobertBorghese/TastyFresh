#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

class Child: public Base {
public:
	Child(int a, int b);

	void DoThing(int a);

	int b = 0;
};

#endif