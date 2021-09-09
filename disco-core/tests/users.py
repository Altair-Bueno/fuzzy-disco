import requests

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


def test_api_users():
    print('create test user:')
    body = """
    {
        "alias": "somecoolalias",
        "password": "somecoolpassword",
        "email": "some@cool.email"
    }
    """
    r = create_user(body)
    print(r.json())

    print('Check if the user has been created:')
    r = get_basic_user_data("somecoolalias")
    print(r.json())

    print('using email for log in:')
    body = """
    {
        "password": "somecoolpassword",
        "email": "some@cool.email"
    }
    """
    r = email_log_in(body)
    print(r.json())

    # alias log in
    print('using alias for log in:')
    old_user_login = """
    {
        "alias": "somecoolalias",
        "password": "somecoolpassword"
    }
    """
    r = alias_log_in(old_user_login)
    print(r.json())

    print('starting auth queries:')
    bearer_token = r.json()['access_token']
    refresh_token = r.json()['refresh_token']
    auth_header = {
        "Authorization": ("Bearer " + bearer_token),
        "Content-Type": "application/json; charset=utf-8"
    }
    print(auth_header)

    # get full user info
    print('get the full user info:')
    r = get_full_user_data(auth_header)
    print(r.json())

    print('refresh token log in. Refresh token: ' + refresh_token)
    r = refresh_token_log_in('{"refresh_token": "' + refresh_token + '" }')
    print(r.json())

    auth_header = {
        "Authorization": ("Bearer " + r.json()['access_token']),
        "Content-Type": "application/json; charset=utf-8"
    }

    # update password
    # NOTE: althought the session has been closed, we can still log in using
    # the token
    print("change user's password:")
    body = """
    {
        "password": "somecoolpassword",
        "new_password": "newpassworddd"
    }
    """
    r = change_password(body, auth_header)
    print(r.status_code)

    # old login should fail
    print('old login info should fail with 4xx code:')
    r = alias_log_in(old_user_login)
    print(f'4xx: {r.status_code}')

    print('Old refresh token should fail:')
    r = refresh_token_log_in('{"refresh_token": "' + refresh_token + '" }')
    print(r)

    print('change user email:')
    body = """
    {
        "email": "the@email.com"
    }
    """
    r = change_user_info(body, auth_header)
    print(r)
    r = get_full_user_data(auth_header)
    print(r.json())

    print('change user description')
    body = """
    {
        "description": "The coolest description"
    }
    """
    r = change_user_info(body, auth_header)
    print(r)
    r = get_full_user_data(auth_header)
    print(r.json())

    # delete user
    print('delete the test user:')
    r = delete_user(auth_header)
    print(r.json())
