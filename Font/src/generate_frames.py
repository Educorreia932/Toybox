import cv2

def generate_frames():
    count = 0
    vidcap = cv2.VideoCapture("../../video.mp4")
    success, image = vidcap.read()
    success = True

    while success:
        vidcap.set(cv2.CAP_PROP_POS_MSEC, (count * 1000))       

        cv2.imwrite(f"../frames/frame_{count}.bmp", image) 

        success, image = vidcap.read()

        count += 1

if __name__ == "__main__":
    generate_frames()