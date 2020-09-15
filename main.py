from math import cos, sin, pi

import datetime
import tkinter as tk 

def _create_circle(self, x, y, radius, **kwargs):
    return self.create_oval(x - radius, y - radius, x + radius, y + radius, **kwargs)

tk.Canvas.create_circle = _create_circle

root = tk.Tk()  
# root.attributes('-fullscreen', True)

SCREEN_WIDTH = root.winfo_screenwidth()
SCREEN_HEIGHT = root.winfo_screenheight()

canvas = tk.Canvas(root)
canvas.pack(fill="both", expand=True)

canvas.create_circle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, SCREEN_HEIGHT / 2, width = 5)

now = datetime.datetime.now()
hour = now.hour % 12
percentage = hour / 12 
angle = percentage * 2 * pi - pi / 2 

print(sin(angle))

x = cos(angle) * SCREEN_HEIGHT / 2 + SCREEN_WIDTH / 2
y = sin(angle) * SCREEN_HEIGHT / 2 + SCREEN_HEIGHT / 2

canvas.create_line(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, x, y, width = 5)

root.mainloop() 