#!/usr/bin/python3
from __future__ import print_function

import requests

from bs4 import BeautifulSoup as bs4

from apis.tasks import set_tasks
from apis.sheets import set_sheets_data


def get_ingredients(url):
    r = requests.get(url)
    data = r.text.encode('utf-8').decode('ascii', 'ignore')
    soup = bs4(data, 'html.parser')
    spans = soup.find_all("span", attrs={"itemprop": "ingredients"})
    values = []
    for span in spans:
        values.append([0, span.get_text()])
    return values


def main():
    urls = [
        "http://www.eatingwell.com/recipe/265329/shrimp-and-cucumber-salad-with-creamy-avocado-dressing/",
        "http://www.eatingwell.com/recipe/262573/quinoa-power-salad/"
    ]
    task_list_id = 'MTY2Mzc4NDIyNjc0NTYxMzM1Njg6NTMxNDY3ODc0NTUzMDc1ODow'

    sheet_id = '1mKoz6etF993f0bC0RXu3i5FdYJRAYr3QXAbjmYyPumk'
    sheet_name = 'Sheet1'
    sheet_range = f'{sheet_name}!B1:C1'

    values = get_ingredients(urls[0])
    ingredients_list = get_ingredients(urls[1])

    set_sheets_data(sheet_id, sheet_range, values)

    for ingredient in ingredients_list:
        task = {'title': ingredient[1]}
        set_tasks(task_list_id, task)


if __name__ == "__main__":
    main()
