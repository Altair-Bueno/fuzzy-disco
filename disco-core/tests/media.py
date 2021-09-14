import requests

import payloads
import users

_URL = 'http://127.0.0.1:8000/api/media/'


def upload_media(file: str, auth_headers: dict[str, str]):
    with open(file, 'rb') as f:
        return requests.post(_URL + 'upload', data=f, headers=auth_headers)


def download_media(id: str, headers: dict[str, str]):
    return requests.get(_URL + id, headers=headers)


def test_media_upload():
    print('Create user and log in')
    body = payloads.new_user('cool', 'a@a.com', '12341234')
    users.create_user(body)
    body = payloads.login_alias('cool', '12341234')
    r = users.alias_log_in(body)
    auth_header = payloads.auth_header(r.json()['access_token'])

    print("Uploading media...")
    r = upload_media(
        'resources/dog-puppy-on-garden-royalty-free-image-1586966191.jpg',
        auth_header)
    print(f'Uploaded file: {r.json()}')

    r = download_media(r.json()['key'], auth_header)
    if r.ok:
        print(f"File download without claiming shouldn't be allowed: {r.text}")
    r = upload_media('test.py', auth_header)
    if r.ok:
        print(f"Text file shouldn't be allowed: {r.json()}")
    users.delete_user(auth_header)

    print('Media test completed')


if __name__ == '__main__':
    test_media_upload()
