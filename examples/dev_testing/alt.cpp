#include "alt.hpp"

Base::Base(int a) {
	baseA = this;
}

void Base::aaa() { }

Base* Base::bbb() {
	return baseA;
}