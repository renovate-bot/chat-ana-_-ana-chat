from requests import post

post("http://localhost:8000/user/create", headers={
    "name": "a",
    "email": "a",
    "profile_image": "a"
}).raise_for_status()

post("http://localhost:8000/server/create", headers={
    "name": "a"
}).raise_for_status()

post("http://localhost:8000/server/join", headers={
    "servername": "a",
    "username": "a"
}).raise_for_status()
