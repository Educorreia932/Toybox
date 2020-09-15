from math import cos, sin, pi

import datetime
import tkinter as tk 

def _create_circle(self, x, y, radius, **kwargs):
    return self.create_oval(x - radius, y - radius, x + radius, y + radius, **kwargs)

tk.Canvas.create_circle = _create_circle

root = tk.Tk()  
root.attributes('-fullscreen', True)
root.title("Clock")

SCREEN_WIDTH = root.winfo_screenwidth()
SCREEN_HEIGHT = root.winfo_screenheight()

canvas = tk.Canvas(root)
canvas.pack(fill="both", expand=True)

radius = SCREEN_HEIGHT / 2 - 50

def draw_hour_handle(now):
    hour = now.hour % 12 + now.minute / 60 
    percentage = hour / 12 
    angle = percentage * 2 * pi - pi / 2 

    x = cos(angle) * radius * 0.60 + SCREEN_WIDTH / 2
    y = sin(angle) * radius * 0.60 + SCREEN_HEIGHT / 2
    
    canvas.create_line(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, x, y, width = 10)
    
def draw_minute_handle(now):
    minutes = now.minute + now.second / 60 
    percentage = minutes / 60
    angle = percentage * 2 * pi - pi / 2 

    x = cos(angle) * radius * 0.87 + SCREEN_WIDTH / 2
    y = sin(angle) * radius * 0.87 + SCREEN_HEIGHT / 2
    
    canvas.create_line(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, x, y, width = 5)
    
def draw_second_handle(now):
    seconds = now.second
    percentage = seconds / 60
    angle = percentage * 2 * pi - pi / 2 

    x = cos(angle) * radius * 0.90 + SCREEN_WIDTH / 2
    y = sin(angle) * radius * 0.90 + SCREEN_HEIGHT / 2
    
    canvas.create_line(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, x, y, width = 3, fill='red')
    
def draw_ticks():
    for i in range(12):
        hour = i % 12
        percentage = hour / 12 
        angle = percentage * 2 * pi - pi / 2 
        
        x0 = cos(angle) * radius * 0.85 + SCREEN_WIDTH / 2
        y0 = sin(angle) * radius * 0.85 + SCREEN_HEIGHT / 2
    
        x1 = cos(angle) * radius + SCREEN_WIDTH / 2
        y1 = sin(angle) * radius + SCREEN_HEIGHT / 2
        
        canvas.create_line(x0, y0, x1, y1, width = 2)
    
def draw():
    now = datetime.datetime.now()
    
    canvas.create_circle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, radius, width = 5)
    draw_hour_handle(now)
    draw_minute_handle(now)
    draw_second_handle(now)
    draw_ticks()

def tick():
    canvas.delete(tk.ALL)
    draw()        
    canvas.after(66, tick) # 30 FPS Update rate

tick()

tk.mainloop()