#!/usr/bin/python3
from __future__ import print_function

from apiclient import discovery
from httplib2 import Http
from oauth2client import file, client, tools
from pathlib import Path

SCOPES = [
    'https://www.googleapis.com/auth/drive.readonly.metadata',
    'https://www.googleapis.com/auth/tasks.readonly',
    'https://www.googleapis.com/auth/tasks',
    'https://www.googleapis.com/auth/spreadsheets.readonly',
    'https://www.googleapis.com/auth/spreadsheets'
]

storage = Path('apis/config/storage.json')
credentials = Path('apis/config/credentials.json')


def get_service(api, version):
    store = file.Storage(storage)
    creds = store.get()

    if not creds or creds.invalid:
        flow = client.flow_from_clientsecrets(credentials, SCOPES)
        creds = tools.run_flow(flow, store)
    SERVICE = discovery.build(api, version, http=creds.authorize(Http()))
    return SERVICE
