import requests

from users import alias_log_in
from users import create_user
from users import delete_user

_URL = 'http://127.0.0.1:8000/api/sessions/'


def get_sessions(auth_headers: dict[str, str]):
    return requests.get(_URL, headers=auth_headers)


def delete_all_sessions(auth_headers: dict[str, str]):
    return requests.post(_URL + 'delete', headers=auth_headers)


def test_api_sessions():
    print('test start')

    body = """
    {
        "email":"some@email.com",
        "alias": "other-alias",
        "password": "otherpassword"
    }
    """
    print(create_user(body))

    r = None
    for i in range(0, 5):
        body = """
        {
            "alias": "other-alias",
            "password": "otherpassword"
        }
        """
        r = alias_log_in(body)

    print('generated 5 sessions')
    bearer_token = r.json()['access_token']
    auth_header = {
        "Authorization": ("Bearer " + bearer_token),
        "Content-Type": "application/json; charset=utf-8"
    }
    print('get all sessions:')
    print(get_sessions(auth_header).json())

    print('Delete sessions:')
    print(delete_all_sessions(auth_header))

    delete_user(auth_header)
