#include "Properties.h"

Properties::Properties() {

}

Properties::Properties(float radius, int numberPoints, int numberLines, int opacity) {
	this->radius = radius;
	this->numberPoints = numberPoints;
	this->numberLines = numberLines;
	this->opacity = opacity;
}