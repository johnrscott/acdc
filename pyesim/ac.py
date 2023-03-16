import pyesim as esim
import matplotlib.pyplot as plt
import numpy as np

ac = esim.LinearAcSweep(0.1,1e6,10001)

## Low pass filter
ac.add_resistor(2, 1, 1e3)
ac.add_capacitor(1, 0, 100e-9)

## Source voltage
ac.add_independent_voltage_source(2, 0, 5, 0)
freq, magnitude, phase = ac.solve()

m = np.array([item[0] for item in magnitude])
f = np.array(freq)

plt.loglog(freq, m/5)

plt.show()
