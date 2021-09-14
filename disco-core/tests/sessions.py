import requests

import payloads
from users import alias_log_in
from users import create_user
from users import delete_user

_URL = 'http://127.0.0.1:8000/api/sessions/'


def get_sessions(auth_headers: dict[str, str]):
    return requests.get(_URL, headers=auth_headers)


def delete_all_sessions(auth_headers: dict[str, str]):
    return requests.post(_URL + 'delete', headers=auth_headers)


def test_api_sessions():
    # Start
    print('Creating sessions')
    body = payloads.new_user('cool', 'a@a.com', '12341234')
    create_user(body)
    body = payloads.login_alias('cool', '12341234')
    r = None
    for _ in range(0, 5):
        r = alias_log_in(body)
        if not r.ok:
            print(f'Failed to log in: {r.text}')
    auth_header = payloads.auth_header(r.json()['access_token'])
    print(f'Get all user sessions: {get_sessions(auth_header).json()}')
    r = delete_all_sessions(auth_header)
    if not r.ok:
        print(f'Failed to delete sessions: {r.text}')
    print(f'Should be an empty list: {get_sessions(auth_header).json()}')

    delete_user(auth_header)


if __name__ == '__main__':
    test_api_sessions()
