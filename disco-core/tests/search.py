import urllib.parse

import requests

_basic_header = {
    "Content-Type": "application/json; charset=utf-8"
}

_URL = 'http://127.0.0.1:8000/api/search'


def search(s: str,date: str,post_drop=0,post_get=0, user_drop=0, user_get=0):
    return requests.get(
        _URL + f'?s={s}&user.drop={user_drop}&user.get={user_get}&post.drop={post_drop}&post.get={post_get}&date={urllib.parse.quote(date)}')
