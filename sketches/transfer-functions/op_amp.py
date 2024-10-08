from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the drive value to see the difference in the frequency response
# Keep it between 0 and 1
drive = 1.

def generate_s_domain_coefficients(drive):
  r1 = 4700.
  c1 = 4.7e-8
  r2 = drive * 500000 + 51000
  c2 = 5.1e-11

  r1c1 = r1 * c1
  r2c2 = r2 * c2

  a0 = r1c1 * r2c2
  a1 = r1c1 + r2c2
  b1 = r2 * c1 + a1

  return ([a0, b1, 1.], [a0, a1, 1.])

# Get generated s-domain coefficients
num, den = generate_s_domain_coefficients(drive)
print('s-domain coefficients', (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print('z-domain coefficients', (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig1 = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([0, 42])
plt.show()