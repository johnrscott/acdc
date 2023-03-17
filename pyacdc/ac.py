import pyacdc as acdc
import matplotlib.pyplot as plt
import numpy as np

ac = acdc.LinearAcSweep(1e3,6e7,10001)

## Notch filter
ac.add_resistor(1, 0, 22)
ac.add_capacitor(2, 1, 303e-12)
ac.add_inductor(3, 1, 30e-6)
ac.add_resistor(2, 3, 0.6)

## Source voltage
ac.add_independent_voltage_source(2, 0, 5, 0)
freq, magnitude, phase = ac.solve()

m = np.array(magnitude[0]) / 5
f = np.array(freq)

plt.loglog(freq, m/5)

plt.show()
