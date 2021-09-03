#pragma once

#include "Properties.h"

#include <vector>

#include <SFML/Graphics.hpp>

#define PI 3.14159265

using namespace std;

class Render {
	private:
		Properties properties;
		sf::RenderWindow* window;

		vector<sf::Vector2f> points;
		vector<pair<int, int>> lines;
	public:
		Render(Properties properties, string imageName);

		vector<sf::Vector2f> getPixelsInLine(sf::Vector2f& startCoordinates, sf::Vector2f& endCoordinates);
		vector<sf::Vector2f> generatePoints();
		vector<pair<int, int>> generateLines(vector<sf::Vector2f>& points, sf::Image& image);
		void drawLine(sf::Vector2f& startCoordinates, sf::Vector2f& endCoordinates);

		void draw();
};
