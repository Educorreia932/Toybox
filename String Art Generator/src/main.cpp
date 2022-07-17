#include "Render.h"

#include <cmath>
#include <iostream>
#include <thread>

using namespace std;

int main() {
	Properties properties;
	Render render(properties, "eva.jpg");

	render.draw();

	return 0;
}

