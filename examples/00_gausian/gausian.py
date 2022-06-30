import imp


import sys
import numpy as np
import cv2

src = cv2.imread("resources/lovely.jpg", cv2.IMREAD_UNCHANGED)


gaus = cv2.GaussianBlur(src, (0, 0), 3)
blur = cv2.blur(src, (7, 7))

cv2.imshow("lovely", src)
cv2.imshow("GaussianBlur", gaus)
cv2.imshow("blur", blur)
cv2.waitKey()