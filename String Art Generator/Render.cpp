#include "Render.h"

#include <iostream>

#define WIDTH 800
#define HEIGHT 800

Render::Render(Properties properties, string imageName) {
	this->properties = properties;

	sf::ContextSettings settings;
	settings.antialiasingLevel = 8;

	window = new sf::RenderWindow(sf::VideoMode(WIDTH, HEIGHT), "String Art", sf::Style::Default, settings);

	points = generatePoints();

	sf::Image image;

	if (!image.loadFromFile(imageName))
		cerr << "Couldn't read image";

	lines = generateLines(points, image);
}

vector<sf::Vector2f> Render::getPixelsInLine(sf::Vector2f& startCoordinates, sf::Vector2f& endCoordinates) {
	bool steep = false;
	float x0 = startCoordinates.x;
	float y0 = startCoordinates.y;
	float x1 = endCoordinates.x;
	float y1 = endCoordinates.y;

	vector<sf::Vector2f> pixels;

	if (abs(x0 - x1) < abs(y0 - y1)) {
		swap(x0, y0);
		swap(x1, y1);
		steep = true;
	}

	if (x0 > x1) {
		swap(x0, x1);
		swap(y0, y1);
	}

	int dx = x1 - x0;
	int dy = y1 - y0;
	int derror2 = std::abs(dy) * 2;
	int error2 = 0;
	int y = y0;

	for (int x = x0; x <= x1; x++) {
		if (steep) {
			pixels.push_back(sf::Vector2f(y, x));
		}

		else {
			pixels.push_back(sf::Vector2f(x, y));
		}

		error2 += derror2;

		if (error2 > dx) {
			y += (y1 > y0 ? 1 : -1);
			error2 -= dx * 2;
		}
	}

	return pixels;
}

vector<sf::Vector2f> Render::generatePoints() {
	vector<sf::Vector2f> points;
	const double divisionAngle = 2 * PI / properties.numberPoints;

	for (int i = 0; i < properties.numberPoints; i++) {
		float currentAngle = divisionAngle * i;

		points.push_back(sf::Vector2f(cos(currentAngle) * (properties.radius - 1) + 400, sin(currentAngle) * (properties.radius - 1) + 400));
	}

	return points;
}

vector<pair<int, int>> Render::generateLines(vector<sf::Vector2f>& points, sf::Image& image) {
	vector<pair<int, int>> lines;

	int choosenPointIndex = 0;

	vector<sf::Vector2f> pixelsToErase;

	// Number of strings
	for (int i = 0; i < properties.numberLines; i++) {
		int startPointIndex = choosenPointIndex;
		float bestScore = 0;

		// Iterate over every point to see where to connect the string to
		for (int j = 0; j < points.size(); j++) {
			vector<sf::Vector2f> pixels = getPixelsInLine(points[startPointIndex], points[j]);
			float score = 0;

			for (sf::Vector2f pixel : pixels) {
				sf::Color color = image.getPixel(pixel.x, pixel.y);

				if (0.3 * color.r + 0.59 * color.g + 0.11 * color.r < 10)
					score += 1;
			}

			if (score > bestScore) {
				choosenPointIndex = j;
				bestScore = score;
				pixelsToErase = pixels;
			}
		}

		int endPointIndex = choosenPointIndex;
		pair<int, int> line = { startPointIndex, endPointIndex };

		lines.push_back(line);

		for (sf::Vector2f pixel : pixelsToErase)
			image.setPixel(pixel.x, pixel.y, sf::Color::White);
	}

	return lines;
}

void Render::drawLine(sf::Vector2f& startCoordinates, sf::Vector2f& endCoordinates) {
	sf::Vertex line[] = {
		sf::Vertex(startCoordinates, sf::Color(0, 0, 0, properties.opacity)),
		sf::Vertex(endCoordinates, sf::Color(0, 0, 0, properties.opacity))
	};

	window->draw(line, 2, sf::Lines);
}

void Render::draw() {
	int count = 0;
	int index = 0;

	sf::Clock clock;

	while (window->isOpen()) {
		sf::Event event;

		window->clear(sf::Color::White);

		while (window->pollEvent(event))
			if (event.type == sf::Event::Closed)
				window->close();

		if (clock.getElapsedTime().asMilliseconds() > 5 && index < properties.numberLines) {
			index++;
			clock.restart();
		}

		for (int i = 0; i < index; i++)
			drawLine(points[lines[i].first], points[lines[i].second]);

		window->display();
	}
}