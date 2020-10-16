#!/usr/bin/python3
from __future__ import print_function

from apis.service import get_service

API = 'sheets'
VERSION = 'v4'
SERVICE = get_service(API, VERSION)


def set_sheets_data(sheet_id, sheet_range, values):
    body = {'values': values}
    sheet = SERVICE.spreadsheets()
    result = sheet.values().append(
        spreadsheetId=sheet_id,
        range=sheet_range,
        valueInputOption="USER_ENTERED",
        body=body).execute()


def get_sheets_data(sheet_id, sheet_range):
    sheet = SERVICE.spreadsheets()
    result = sheet.values().get(
        spreadsheetId=sheet_id, range=sheet_range).execute()
    values = result.get('values', [])

    if not values:
        print('No data found.')
    else:
        for row in values:
            print(f'{row[0]}, {row[1]}')


if __name__ == '__main__':
    get_sheets_data(
        sheet_id='1mKoz6etF993f0bC0RXu3i5FdYJRAYr3QXAbjmYyPumk',
        sheet_range='Sheet1!A2:B')
