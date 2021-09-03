#pragma once

#include <iostream>

struct Properties {
	float radius = 400;
	int numberPoints = 200;
	int numberLines = 20000;
	int opacity = 40;
};

//ostream& operator<<(ostream &os, const Properties &p) {
//	os << "Radius: " << p.radius << endl
//		<< "Number of points: " << p.numberPoints << endl
//		<< "Number of lines: " << p.numberLines << endl
//		<< "Opacity: " << p.opacity << endl;
//
//	return os;
//};
