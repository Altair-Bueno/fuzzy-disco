import urllib.parse
import requests
_basic_header = {
    "Content-Type": "application/json; charset=utf-8"
}

_URL = 'http://127.0.0.1:8000/api/search'


def search(s:str,drop:int,get:int,date:str):
    return requests.get(_URL + f'?s={s}&drop={drop}&get={get}&date={urllib.parse.quote(date)}')
