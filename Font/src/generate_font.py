import fontforge
import os

def generate_font():
    font = fontforge.font()

    font.encoding = "UnicodeFull"
    font.familyname = "BadApple"
    font.fontname = "BadApple"
    font.fullname = "BadAppleRegular"

    codepoint = 0xF0000
    frames = os.listdir("../frames")

    for frame in frames:
        glyph = font.createMappedChar(codepoint)
        glyph.importOutlines(f"../frames/{frame}", ('removeoverlap', 'correctdir'))
        glyph.removeOverlap()
        
        codepoint += 1

    font.generate("../out/bad_apple.woff")

if __name__ == "__main__":
    generate_font()
