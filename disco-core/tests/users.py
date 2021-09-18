import urllib.parse

import requests

import media
import payloads

_basic_header = {
    "Content-Type": "application/json; charset=utf-8"
}
_URL = 'http://127.0.0.1:8000/api/users/'


def create_user(body: str):
    return requests.post(_URL + 'auth/signup', body, headers=_basic_header)


def get_basic_user_data(username: str):
    return requests.get(_URL + username)


def get_full_user_data(auth_header: dict[str, str]):
    return requests.get(_URL, headers=auth_header)


def email_log_in(body: str):
    return requests.post(_URL + 'auth/login?using=email', body,
                         headers=_basic_header)


def refresh_token_log_in(body: str):
    return requests.post(_URL + 'auth/login?using=refresh_token', body,
                         headers=_basic_header)


def alias_log_in(body: str):
    return requests.post(_URL + 'auth/login?using=alias', body,
                         headers=_basic_header)


def change_password(body: str, auth_header: dict[str, str]):
    return requests.post(_URL + 'update/password', body, headers=auth_header)


def change_user_info(body: str, auth_header: dict[str, str]):
    return requests.post(_URL + 'update', body, headers=auth_header)


def delete_user(auth_header: dict[str, str]):
    return requests.delete(_URL, headers=auth_header)


def change_user_avatar(image_id: str, auth_header: dict[str, str]):
    return requests.post(_URL + 'update/avatar',
                         '{"mediaid": "' + image_id + '" }',
                         headers=auth_header)


def get_public_posts(id: str,date: str, block = 0):
    return requests.get(
        _URL + f'{id}/posts?block={block}&date={urllib.parse.quote(date)}',
        headers=_basic_header)


def get_private_posts(id: str, date: str, auth_header: dict[str,str], block = 0):
    return requests.get(
        _URL + f'{id}/posts?private&block={block}&date={urllib.parse.quote(date)}',
        headers=auth_header)


def easy_refresh_login(refresh_token: str):
    p = payloads.login_refresh_token(refresh_token)
    access_token = refresh_token_log_in(p).json()['access_token']
    header = payloads.auth_header(access_token)
    return header


def test_api_users():
    # Create a user
    body = payloads.new_user('hello', 'a@gmail.com', '12341234')
    r = create_user(body)
    print(f'Created user: {r.json()}')

    # Get back the user data
    r = get_basic_user_data("hello")
    print(f'Get basic data: {r.json()}')

    body = payloads.login_email('a@gmail.com', '12341234')
    r = email_log_in(body)
    if not r.ok:
        print('Email log in failed')

    # alias log in
    old_user_login = payloads.login_alias('hello', '12341234')
    r = alias_log_in(old_user_login)
    if not r.ok:
        print('Alias log in failed')
    # Auth queries
    refresh_token = r.json()['refresh_token']
    auth_header = easy_refresh_login(refresh_token)

    # get full user info
    r = get_full_user_data(auth_header)
    print(f'Full user info: {r.json()}')

    body = payloads.change_password('12341234', '123412344')
    r = change_password(body, auth_header)
    if not r.ok:
        print('password change failed')

    # old login should fail
    r = alias_log_in(old_user_login)
    if r.ok:
        print(f'Alias log in did not fail. Expected 4xx code: {r.text}')

    r = refresh_token_log_in(payloads.login_refresh_token(refresh_token))
    if r.ok:
        print(f'Refresh token should have failed: {r.text}')

    # First stage completed

    r = alias_log_in(payloads.login_alias('hello', '123412344'))
    auth_header = payloads.auth_header(r.json()['access_token'])
    refresh_token = r.json()['refresh_token']

    r = change_user_info(payloads.change_user_info('cool@gmail.com'),
                         auth_header)
    if not r.ok:
        print(f'Failed to change user info: {r.text}')
    r = get_full_user_data(auth_header)
    if (not r.ok) or (r.json()['email'] != 'cool@gmail.com'):
        print(f'Failed to change user data: {r.text}')

    auth_header = easy_refresh_login(refresh_token)

    # Second stage completed

    r = media.upload_media('resources/photo-1491604612772-6853927639ef.jpeg',
                           auth_header)
    r = change_user_avatar(r.json()['key'], auth_header)
    if not r.ok:
        print(f'Failed to change user avatar: {r.text}')

    # delete user
    print('delete the test user:')
    r = delete_user(auth_header)
    if not r.ok:
        print(f'Failed to delete user: {r.text}')

    print('User test completed')


if __name__ == '__main__':
    test_api_users()
