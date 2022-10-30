import os

def generate_file():
    contents = "".join(chr(0xF0000 + i) for i in range(len(os.listdir("../frames"))))

    f = open("../out/bad_apple.txt", "w", encoding="utf-8")
    f.write(contents)
    f.close()

if __name__ == "__main__":
    generate_file()