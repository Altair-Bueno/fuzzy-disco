import requests

import media
import payloads
import users

_URL = 'http://127.0.0.1:8000/api/posts/'


def create_post(body: str, auth_headers: dict[str, str]):
    return requests.post(_URL + 'new', body, headers=auth_headers)


def get_post(id: str, headers: dict[str, str]):
    return requests.get(_URL + id, headers=headers)


def delete_post(id: str, auth_headers: dict[str, str]):
    return requests.delete(_URL + id, headers=auth_headers)


def edit_post(id: str, body: str, auth_headers: dict[str, str]):
    return requests.patch(_URL + f'{id}', body, headers=auth_headers)


def test_posts_api():
    # start
    print('Create user and log in')
    body = payloads.new_user('hello', 'a@a.com', '12341234')
    users.create_user(body)
    body = payloads.login_alias('hello', '12341234')
    r = users.alias_log_in(body)
    auth_header = payloads.auth_header(r.json()['access_token'])

    # Upload media
    image = \
    media.upload_media('resources/photo-1491604612772-6853927639ef.jpeg',
                       auth_header).json()['key']
    audio = media.upload_media('resources/file_example_MP3_700KB.mp3',
                               auth_header).json()['key']
    # Create post
    body = payloads.new_post('New post', 'This is a post', image, audio,
                             payloads.VISIBILITY_PUBLIC)
    r = create_post(body, auth_header)
    if not r.ok:
        print(f"Post creation went wrong: {r.text}")

    # Get the post
    print('Get the post')
    post_id = r.json()['post_id']
    r = get_post(post_id, auth_header)
    if r.ok:
        print(r.json())
    else:
        print(f"Failed to retrieve post: {r.text}")

    body = payloads.edit_post(payloads.VISIBILITY_PRIVATE)
    r = edit_post(post_id, body, auth_header)
    if not r.ok:
        print(f"Failed to edit post: {r.text}")
    users.delete_user(auth_header)


if __name__ == '__main__':
    test_posts_api()
