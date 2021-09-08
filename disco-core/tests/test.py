from sessions import test_api_sessions
from users import test_api_users


def main():
    test_api_users()
    test_api_sessions()


if __name__ == '__main__':
    main()
