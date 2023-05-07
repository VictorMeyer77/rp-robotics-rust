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
steps = [5.85, 5.9, 5.95, 6.0]
current_step = -1

def increase(target):
    while i < target:
        print(i)
        p.ChangeDutyCycle(i)
        time.sleep(0.05)
        i += 0.02

def decrease(target):
    while i > target:
        print(i)
        p.ChangeDutyCycle(i)
        time.sleep(0.05)
        i -= 0.02

while True:
    s = input("commnd")
    print(s)
    if s == "a" and current_step < 3:
       current_step += 1
       increase(steps[current_step])
    elif s == "z" and current_step > 0:
        current_step -= 1
        decrease(steps[current_step])
    elif s == "p":
        break