import imp


import sys
import numpy as np
import cv2

src = cv2.imread("resources/lovely.jpg", cv2.IMREAD_UNCHANGED)

# kernel = np.array([[1/9,1/9,1/9],
#                    [1/9,1/9,1/9],
#                    [1/9,1/9,1/9]], dtype=np.float32)

kernel = np.ones((4, 4), dtype = np.float32)/9.

dst = cv2.filter2D(src, -1 , kernel)

cv2.imshow("lovely", src)
cv2.imshow("dst", dst)
cv2.waitKey()