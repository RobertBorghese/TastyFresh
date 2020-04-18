#include "main.hpp"

Base::Base() { }

Child::Child() { }

int main() {
	int myInt = 10;
	int myFloat = 20.0f;
	double myDouble = 30.0;
	Child* myChild = new Child();

	int castedToInt = (int)(myFloat);
	int castedToInt2 = (int)(myDouble);
	int castedToInt3 = (int)(12.0f);

	double castedToDouble = (double)(myInt);
	double castedToDouble2 = (double)(myFloat);
	double castedToDouble3 = (double)(100);

	Base* castedToBase = (Base*)(myChild);
	void* castedToPointer = (void*)(0x000123);
	Base castedToValue = (Base)(myChild);

	Base* staticCastToBase = static_cast<Base*>(myChild);
	Base* dynamicCastToBase = dynamic_cast<Base*>(myChild);
	Base* reinterpretCastToBase = reinterpret_cast<Base*>(myChild);

	delete myChild;
}