import cv2
import sys

HEIGHT = 40

def put_text(img, text, line):
    cv2.putText(img, text, (3, HEIGHT + HEIGHT * line), cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 0, 0))


def mouse_event(event, x, y, flags, param):
    pixel = img[y][x]
    img1 = img.copy()
    put_text(img1, "B: {0}, G:{1}, R: {2}".format(pixel[0],pixel[1],pixel[2]), 3);
    cv2.imshow("Cat", img1)


img = cv2.imread("resources/cat.bmp")

if img is None:
    print("Image load failed")
    sys.exit()

put_text(img, "type: {0}".format(img.dtype), 0)
put_text(img, "Size: {0}".format(img.shape), 1)
put_text(img, "Depth: {0}".format(img.ndim), 2)


cv2.namedWindow("Cat")

cv2.setMouseCallback("Cat", mouse_event)

cv2.imshow("Cat", img)
cv2.waitKey()
cv2.destroyAllWindows()