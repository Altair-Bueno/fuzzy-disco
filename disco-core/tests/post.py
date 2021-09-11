import requests
import users
import media

_URL = 'http://127.0.0.1:8000/api/posts/'


def create_post(body:str,auth_headers:dict[str,str]):
    return requests.post(_URL + 'new',body, headers=auth_headers)


def get_post(id:str,headers:dict[str,str]):
    return requests.get(_URL + id, headers=headers)


def delete_post(id:str,auth_headers:dict[str,str]):
    return requests.delete(_URL + id,headers=auth_headers)


def test_posts_api():
    print('creating user')
    body = """
    {
        "email": "cool@email.com",
        "alias": "aliasss",
        "password": "12345678"
    }
    """
    users.create_user(body)
    body = """
    {
        "alias": "aliasss",
        "password": "12345678"
    }
    """
    r = users.alias_log_in(body)
    bearer_token = r.json()['access_token']
    refresh_token = r.json()['refresh_token']
    auth_header = {
        "Authorization": ("Bearer " + bearer_token),
        "Content-Type": "application/json; charset=utf-8"
    }


    image = media.upload_media('resources/photo-1491604612772-6853927639ef.jpeg',auth_header).json()['key']
    audio = media.upload_media('resources/file_example_MP3_700KB.mp3',auth_header).json()['key']

    body = f"""
    {{
        "title": "Title",
        "caption": "Cool caption",
        "photo": "{image}",
        "audio": "{audio}",
        "visibility": "Public"
    }}
    """
    print('creating post')
    r = create_post(body,auth_header)
    print(f'Should be 2xx: {r}')
    print(r.json())

    print('Get the post')
    postid = r.json()['post_id']
    r = get_post(postid,auth_header)
    print(r.json())

    print('delete post')
    print(f'Should be 2xx code: {delete_post(postid,auth_header)}')

    users.delete_user(auth_header)


if __name__ == '__main__':
    test_posts_api()
