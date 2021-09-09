
import requests
import users

_URL = 'http://127.0.0.1:8000/api/media/'


def upload_media(file: str, auth_headers: dict[str, str]):
    with open(file, 'rb') as f:
        return requests.post(_URL + 'upload', data=f, headers=auth_headers)


def test_media_upload():
    body = """
    {
        "alias": "cooool",
        "email": "random@email.com",
        "password": "passwordddd"
    }
    """
    users.create_user(body)
    r = users.alias_log_in('{"alias": "cooool", "password": "passwordddd"}')
    auth_header = {
        "Authorization": ("Bearer " + r.json()['access_token']),
        "Content-Type": "application/json; charset=utf-8"
    }

    print("Uploading media...")
    r = upload_media(
        'resources/dog-puppy-on-garden-royalty-free-image-1586966191.jpg',
        auth_header)
    print(r.json())

    r = upload_media('resources/photo-1491604612772-6853927639ef.jpeg',
                     auth_header)
    print(r.json())

    r = upload_media(
        'resources/png-transparent-logo-online-and-offline-e-online-design-text-logo-online-and-offline.png',
        auth_header)
    print(r.json())

    r = upload_media('test.py', auth_header)
    print('should fail:')
    print(r)

    users.delete_user(auth_header)

if __name__ == '__main__':
    test_media_upload()