#!/usr/bin/python3
from __future__ import print_function

from service import get_service

API = 'drive'
VERSION = 'v3'
SERVICE = get_service(API, VERSION)


def get_files():
    files = SERVICE.files().list().execute().get('files', [])
    for f in files:
        print(f['name'], f['mimeType'])


def main():

    get_files()


if __name__ == "__main__":
    main()
