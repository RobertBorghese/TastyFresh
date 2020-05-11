#include "main.hpp"

Base::Base(int a) { }

Child::Child(int a, int b): dsadas(a), b(b) {

}

void Child::DoThing(int a) {
	this->a = a;
}