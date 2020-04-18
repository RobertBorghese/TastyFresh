#include "main.hpp"

int MyLineEdit::get_count() {
	return _count;
}

void MyLineEdit::set_count(int v) {
	_count = v;
	repaint();
}