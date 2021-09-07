import requests


def test_api_users(_URL: str):
    basic_header = {
        "Content-Type": "application/json; charset=utf-8"
    }

    # Create test user
    print('create test user')
    body = """
    {
        "alias": "somecoolalias",
        "password": "somecoolpassword",
        "email": "some@cool.email"
    }
    """
    r = requests.post(_URL + '/api/users/auth/signup',body,headers=basic_header)
    print(r.json())
    user_url = _URL + '/api/users/somecoolalias'
    # check if user exist
    print('Check if the user has been created')
    r = requests.get(user_url)
    print(r.json())

    # email log in
    print('using email for log in')
    body = """
    {
        "password": "somecoolpassword",
        "email": "some@cool.email"
    }
    """
    r = requests.post(_URL + '/api/users/auth/login?using=email',body,headers=basic_header)
    print(r.json())

    # alias log in
    print('using alias for log in')
    old_user_login = """
    {
        "alias": "somecoolalias",
        "password": "somecoolpassword"
    }
    """
    r = requests.post(_URL + '/api/users/auth/login?using=alias',old_user_login,headers=basic_header)
    print(r.json())
    # Auth queries
    print('starting auth queries')
    bearer_token = r.json()['access_token']
    auth_header = {
        "Authorization": ("Bearer " + bearer_token),
        "Content-Type": "application/json; charset=utf-8"
    }
    print(auth_header)

    # get full user info
    print('get the full user info')
    r = requests.get(_URL + '/api/users/',headers=auth_header)
    print(r.json())

    # update password
    # NOTE: althought the session has been closed, we can 
    print("change user's password")
    body = """
    {
        "password": "somecoolpassword",
        "new_password": "newpassworddd"
    }
    """
    r = requests.put(_URL + '/api/users/update/password',body, headers=auth_header)
    print(r.status_code)

    # old login should fail
    print('old login info should fail with 4xx code')
    r = requests.post(_URL + '/api/users/auth/login?using=alias',old_user_login)
    print(r.status_code)

    # delete user
    print('delete the test user')
    r = requests.delete(_URL + '/api/users/', headers=auth_header)
    print(r.json())