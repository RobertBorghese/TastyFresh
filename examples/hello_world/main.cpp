#include "examples/hello_world/main.hpp"

int bla = 32;

std::shared_ptr<Asd> anotherTest = nullptr;


#include <iostream>
#include <vector>

void main() {
	float bla1 = test(12);
	float bla2 = test("fdsfds");
	int bla3 = test(23.43f);
	char bla4 = test(1.2);

	char** bla5 = &&bla4;
	char**** bla6 = &&&&bla4;

	char bla7 = bla4;

	char** bla10 = bla5;

	auto test = fdjsklfds();

	bool cond = true;

	auto test = Bla();
	{
		int one = 1;
		{
			int two = 2;
			{
				int three = 4;
				int another = 32;
			}
		}
	}

	if(cond) {
		return 3;
	}

	std::tuple<int, int> test2 = std::make_tuple(32, 54);

	std::cout << "teste" << std::endl;
	int b = 32 == 32 ? 1 : 0;

	return 10;

	std::vector<int> bla(34, "fdsfds");
}

void aaaaa(map<int> a) { }

int test() {
	return 3;
}

float test(const char* a) {
	return 0.0f;
}

float test(int a) {
	return 0.0f;
}

int test(float a) {
	return 0.0f;
}

char test(double a) {
	return 0.0f;
}