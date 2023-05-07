import time

import RPi.GPIO as GPIO



GPIO.setmode(GPIO.BOARD)
GPIO.setup(12,GPIO.OUT)

p = GPIO.PWM(12, 50)

p.start(0)
print("Starting 0")
time.sleep(5)


p.ChangeDutyCycle(3)
print("3")
time.sleep(5)

start = 4
max = 5.75

try:

    while True:
        i = start
        while i < max:

            print(i)
            p.ChangeDutyCycle(i)
            time.sleep(0.05)
            i += 0.02
        time.sleep(3)
        while i > max:
            print(i)
            p.ChangeDutyCycle(i)
            time.sleep(0.05)
            i -= 0.05

except:
    KeyboardInterrupt