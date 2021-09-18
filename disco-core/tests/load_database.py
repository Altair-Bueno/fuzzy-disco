import media
import users
import payloads
import post
import os
import random
from concurrent.futures import ThreadPoolExecutor


def get_random_audio():
    return get_random_file("resources/audio")


def get_random_image():
    return get_random_file("resources/images")


def get_random_file(path:str):
    file = random.choice(os.listdir(path))
    return f"{path}/{file}"


def load(id:int):
    body = payloads.new_user(f"hello{id}", f"a{id}@uma.es", "12341234")
    r = users.create_user(body)
    if not r.ok:
        print(r.text)
        return -1
    login = payloads.login_alias(f"hello{id}", "12341234")
    r = users.alias_log_in(login)
    auth_header = payloads.auth_header(r.json()["access_token"])
    refresh_token = r.json()["refresh_token"]
    for i in range(0, 3):
        audio = get_random_audio()
        image = get_random_image()
        id_audio = media.upload_media(audio,auth_header).json()["key"]
        id_image = media.upload_media(image,auth_header).json()["key"]
        body = payloads.new_post(f"Sample{id}",f"Test caption {audio} and {image}",id_image,id_audio,payloads.VISIBILITY_PUBLIC)
        post.create_post(body,auth_header)
    return id


def main():
    with ThreadPoolExecutor() as executor:
        l = []
        for i in range(0,40):
            r = executor.submit(lambda n=i: load(n))
            l.append(r)
        for wait in l:
            print(wait.result())


if __name__ == "__main__":
    main()
