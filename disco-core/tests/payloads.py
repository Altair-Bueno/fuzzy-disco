def new_user(alias:str,email:str,password:str):
    return f"""
    {{
        "alias": "{alias}",
        "password": "{password}",
        "email": "{email}"
    }}
    """


def login_email(email:str,password:str):
    return f"""
    {{
        "email": "{email}",
        "password": "{password}"
    }}
    """


def login_alias(alias:str,password:str):
    return f"""
    {{
        "alias": "{alias}",
        "password":"{password}"
    }}
    """


def login_refresh_token(refresh_token:str):
    return f"""
    {{
        "refresh_token":"{refresh_token}"
    }}
    """


def auth_header(bearer_token:str):
    return {
        "Authorization": ("Bearer " + bearer_token),
        "Content-Type": "application/json; charset=utf-8"
    }


def basic_header():
    return {
        "Content-Type": "application/json; charset=utf-8"
    }


def change_password(old:str,new:str):
    return f"""
    {{
        "password": "{old}",
        "new_password": "{new}"
    }}
    """


def change_user_info(email:str):
    return f"""
    {{
        "email": "{email}"
    }}
    """


def new_post(title:str,caption:str,photo:str,audio:str,visibility:str):
    return f"""
    {{
        "title": "{title}",
        "caption": "{caption}",
        "photo": "{photo}",
        "audio":"{audio}",
        "visibility": "{visibility}"
    }}
    """


def edit_post(visibility:str):
    return f"""
    {{
        "visibility": "{visibility}"
    }}
    """


VISIBILITY_PUBLIC = "Public"
VISIBILITY_PRIVATE = "Private"
