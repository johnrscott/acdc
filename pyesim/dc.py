import pyacdc as esim

dc = acdc.LinearDcAnalysis()
dc.add_resistor(1, 0, 100)
dc.add_resistor(2, 1, 100)
dc.add_independent_voltage_source(2, 0, 5, 0)
voltages, currents = dc.solve()

print(f"Voltages: {voltages}")
print(f"Currents: {currents}")
