#!/usr/bin/python

import csv
import math
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import sys
import re

from six import print_

# apply plot style
sns.set()

x = []
y = []

with open(sys.argv[1], 'r') as f:
    print_('Reading data from', sys.argv[1])

    for line in f:
        # Check if line contains I16x3 format (raw EMF log)
        match = re.search(r'I16x3\s*\{\s*x:\s*(-?\d+),\s*y:\s*(-?\d+),\s*z:\s*(-?\d+)\s*\}', line)
        if match:
            x.append(int(match.group(1)))
            y.append(int(match.group(2)))
        else:
            # Fallback to CSV format for backward compatibility
            line = line.strip()
            if line:
                row = line.split('\t')
                if len(row) >= 2 and row[0] and row[1]:
                    try:
                        x.append(int(row[0]))
                        y.append(int(row[1]))
                    except ValueError:
                        continue

r = math.ceil(max(max(np.abs(x)), max(np.abs(y))) / 100) * 100

plt.plot(x, y, '.')
plt.xlim(-r, r)
plt.ylim(-r, r)
plt.gca().set_aspect(1)
plt.tight_layout()

plt.savefig('emf.svg')
plt.close()