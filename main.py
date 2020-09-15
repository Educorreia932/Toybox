import tkinter as tk 

root = tk.Tk()  
root.attributes('-fullscreen', True)

canvas = tk.Canvas(root)
canvas.pack()

canvas.create_oval(100, 100, 300, 300, width = 5)

root.mainloop() 