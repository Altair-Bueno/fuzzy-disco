import urllib.parse

import requests

_basic_header = {
    "Content-Type": "application/json; charset=utf-8"
}

_URL = 'http://127.0.0.1:8000/api/search/'


def user(s: str,date: str,block=0):
    return requests.get(
        _URL + f'user?s={s}&block={block}&date={urllib.parse.quote(date)}')


def post(s:str,date:str,block=0):
    return requests.get(_URL + f'post?s={s}&block={block}&date={urllib.parse.quote(date)}')
