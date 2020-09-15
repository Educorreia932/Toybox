import tkinter as tk 

def _create_circle(self, x, y, radius, **kwargs):
    return self.create_oval(x - radius, y - radius, x + radius, y + radius, **kwargs)

tk.Canvas.create_circle = _create_circle

root = tk.Tk()  
root.attributes('-fullscreen', True)

SCREEN_WIDTH = root.winfo_screenwidth()
SCREEN_HEIGHT = root.winfo_screenheight()

print(SCREEN_WIDTH, SCREEN_HEIGHT)

canvas = tk.Canvas(root)
canvas.pack(fill="both", expand=True)

canvas.create_circle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, SCREEN_HEIGHT / 2, width = 5)

root.mainloop() 