# !/usr/bin/env python3
# -*- coding: utf8 -*-
"""
DESCRIPTION GOES HERE
"""
import os
import sys

import pandas as pd


def main():
    d = {'col1': [1, 2], 'col2': [3, 4]}
    df = pd.DataFrame(data=d)
    print('Hello world!')
    print(d)


if __name__ == '__main__':
    main()
