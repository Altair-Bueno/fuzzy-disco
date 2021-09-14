from media import test_media_upload
from post import test_posts_api
from sessions import test_api_sessions
from users import test_api_users


def main():
    print("\ntesting user API...")
    test_api_users()
    print("\ntesting sessions API...")
    test_api_sessions()
    print("\ntesting media API...")
    test_media_upload()
    print("\ntesting post API...")
    test_posts_api()


if __name__ == '__main__':
    main()
