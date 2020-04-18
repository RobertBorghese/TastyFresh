#ifndef MAIN_TASTYFILE
#define MAIN_TASTYFILE

#include <QLineEdit>

class MyLineEdit: public QLineEdit {
	Q_OBJECT
	Q_PROPERTY(int count READ get_count WRITE set_count)

public:
	int get_count();
	void set_count(int v);

	int _count = 0;
};

#endif