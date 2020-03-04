#include "./examples/hello_world/main.hpp"

void main() {
	int bla1 = test(12);
	float bla2 = test("fdsfds");
	const char* bla3 = test(23.43f);
	QString* bla4 = test(1.2);

	QString** bla5 = &bla4;
	QString**** bla6 = &&&bla4;

	QString bla7 = *bla4;

	QString** bla10 = bla5;

	auto test = fdjsklfds();
}

int test(int a) {
	return 0;
}

float test(const char* a) {
	return 0.0f;
}

const char* test(float a) {
	return "This is text";
}

QString* test(double a) {
	return nullptr;
}