#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import numpy as np
import matplotlib.pyplot as plt

OUT_DIR = os.path.dirname(os.path.abspath(__file__))

def plot_asymptotic_notation():
  """Generate figures for 'Asymptotic Notation' chapter."""
  plt.rc('lines', linewidth=3)
  plt.rc('font', size=14)

  # Fig. 1
  x = np.linspace(1, 10)
  plt.plot(x, np.log(x), label='Logarithmic O(log(n))', linewidth=3)
  plt.plot(x, x, label='Linear O(n)', linewidth=3)
  plt.plot(x, x * np.log(x), label='Linearithmic O(n log(n))', linewidth=3)
  plt.xlabel('Size of Input')
  plt.ylabel('Cost')
  plt.legend()
  plt.savefig(os.path.join(OUT_DIR, 'fig1.png'))
  plt.clf()

  # Fig. 2
  x = np.linspace(1, 30)
  plt.plot(x, 10 * x + 29, label='f(x) = 10x + 29')
  plt.plot(x, x**2 + 1, label='g(x) = x^2 + 1')
  plt.xlabel('Size of Input')
  plt.ylabel('Cost')
  plt.legend()
  plt.savefig(os.path.join(OUT_DIR, 'fig2.png'))
  plt.clf()

  # Fig. 3
  x = np.linspace(1, 10)
  plt.axvline(x=4, linestyle='dashed', color='lightgray', linewidth=2)
  plt.plot(x, 3 * x + 4, label='f(n) = 3n + 4')
  plt.plot(x, 4 * x, label='g(n) = 4n')
  plt.xlabel('Size of Input')
  plt.ylabel('Cost')
  plt.legend()
  plt.savefig(os.path.join(OUT_DIR, 'fig3.png'))
  plt.clf()

  # Fig. 4
  x = np.linspace(1, 10)
  plt.axvline(x=2, linestyle='dashed', color='lightgray', linewidth=2)
  plt.plot(x, 3 * x + 4, label='f(n) = 3n + 4')
  plt.plot(x, x, label='k1 * g(n) = n')
  plt.plot(x, 5 * x, label='k2 * g(n) = 5n')
  plt.xlabel('Size of Input')
  plt.ylabel('Cost')
  plt.legend()
  plt.savefig(os.path.join(OUT_DIR, 'fig4.png'))
  plt.clf()

  # Close current window
  plt.close()

plot_asymptotic_notation()
