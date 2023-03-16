import pyesim as esim

ac = esim.LinearAcSweep(1,10,11)
ac.add_resistor(1, 0, 100)
ac.add_resistor(2, 1, 100)
ac.add_independent_voltage_source(2, 0, 5, 0)
freq, voltages, currents = ac.solve()

print(f"frequencies {freq}")
print(f"Voltages: {voltages}")
print(f"Currents: {currents}")
