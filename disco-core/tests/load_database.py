import random

import users
import payloads
import post
from concurrent.futures import ThreadPoolExecutor


def create_dummy_user(i:int):
    body = payloads.new_user(f"dummy{i}",f"dummy{i}@hello.com","12341234")
    return users.create_user(body)


def generate_data():
    with ThreadPoolExecutor() as executor:
        running_tasks = [executor.submit(lambda: create_dummy_user(i)) for i in range(0,10000)]
        for running in running_tasks:
            print(running.result().json())


if __name__ == "__main__":
    generate_data()
