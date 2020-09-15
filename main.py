from math import cos, sin, pi

import datetime
import tkinter as tk 

def _create_circle(self, x, y, radius, **kwargs):
    return self.create_oval(x - radius, y - radius, x + radius, y + radius, **kwargs)

tk.Canvas.create_circle = _create_circle

root = tk.Tk()  
# root.attributes('-fullscreen', True)
root.title("Clock")

SCREEN_WIDTH = root.winfo_screenwidth()
SCREEN_HEIGHT = root.winfo_screenheight()

canvas = tk.Canvas(root)
canvas.pack(fill="both", expand=True)

canvas.create_circle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, SCREEN_HEIGHT / 2, width = 5)

now = datetime.datetime.now()

def draw_hour_handle():
    hour = now.hour % 12 + now.minute / 60 
    percentage = hour / 12 
    angle = percentage * 2 * pi - pi / 2 

    x = cos(angle) * 450 + SCREEN_WIDTH / 2
    y = sin(angle) * 450 + SCREEN_HEIGHT / 2
    
    canvas.create_line(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, x, y, width = 10)
    
def draw_minute_handle():
    minutes = now.minute + now.second / 60 
    percentage = minutes / 60
    angle = percentage * 2 * pi - pi / 2 

    x = cos(angle) * 500 + SCREEN_WIDTH / 2
    y = sin(angle) * 500 + SCREEN_HEIGHT / 2
    
    canvas.create_line(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, x, y, width = 5)
    
def draw_ticks():
    for i in range(12):
        hour = i % 12
        percentage = hour / 12 
        angle = percentage * 2 * pi - pi / 2 
        
        x0 = cos(angle) * 450 + SCREEN_WIDTH / 2
        y0 = sin(angle) * 450 + SCREEN_HEIGHT / 2
    
        x1 = cos(angle) * SCREEN_HEIGHT / 2 + SCREEN_WIDTH / 2
        y1 = sin(angle) * SCREEN_HEIGHT / 2 + SCREEN_HEIGHT / 2
        
        canvas.create_line(x0, y0, x1, y1, width = 2)
    
draw_hour_handle()
draw_minute_handle()
draw_ticks()

root.mainloop() 