#!/usr/bin/python3
from __future__ import print_function

from apis.service import get_service

API = 'tasks'
VERSION = 'v1'
SERVICE = get_service(API, VERSION)


def set_tasks(task_list_id, task):
    result = SERVICE.tasks().insert(tasklist=task_list_id, body=task).execute()


def get_tasks(task_list_id):
    tasks = SERVICE.tasks().list(tasklist=task_list_id).execute()
    for task in tasks['items']:
        print(task['title'])


def get_task_lists():
    results = SERVICE.tasklists().list(maxResults=10).execute()
    items = results.get('items', [])
    task_lists = []
    if not items:
        print('No task lists found.')
    else:
        print('Task lists:')
        for item in items:
            task_lists.append({'id': item['id'], 'name': item['title']})
    print(task_lists)


def main():

    get_task_lists()


if __name__ == "__main__":
    main()
