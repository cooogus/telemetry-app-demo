#!/usr/bin/env python3
import random, time

device = "junos-1"
fans = ["FAN1", "FAN2", "FAN3"]

now_ns = int(time.time_ns())
for f in fans:
    rpm = 7800 + random.randint(-250, 250)
    print(f"fan_speed,device={device},fan={f} rpm={rpm} {now_ns}")