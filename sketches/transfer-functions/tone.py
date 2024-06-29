from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the tone value to see the difference in the frequency response
# Keep it between 0 and 1
tone = 1.

def generate_s_domain_coefficients(tone):
  Cs = 0.22e-6
  Rs = 1e3
  Ri = 10e3
  Cz = 0.22e-6
  Rz = 220.0
  Rf = 1e3

  wp = 1.0 / (Cs * Rs * Ri / (Rs + Ri));
  
  Rl = tone
  Rr = 1. - tone

  wz = 1.0 / (Cz * (Rz + (Rl * Rr / (Rl + Rr))))

  Y = (Rl + Rr) * (Rz + (Rl * Rr / (Rl + Rr)))
  X = (Rr / (Rl + Rr)) / ((Rz + (Rl * Rr / (Rl + Rr))) * Y)
  W = Y / ((Rl * Rf) + Y)

  alpha = (Rl * Rf + Y) / (Y * Rs * Cs)

  return (
    [0., alpha, alpha * W * wz],
    [1., wp + wz + X, wp * wz]
  )

# Get generated s-domain coefficients
num, den = generate_s_domain_coefficients(tone)
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
plt.ylim([-70, 5])
plt.show()