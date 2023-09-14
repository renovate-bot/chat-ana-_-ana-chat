from requests import post

post("http://localhost:8000/server/create", headers={
    "name": "logo"
}).raise_for_status()

for i in ["b", "c"]:
    post("http://localhost:8000/user/create", headers={
        "name": i,
        "email": i,
        "profile_image": i
    }).raise_for_status()

    post("http://localhost:8000/server/create", headers={
        "name": i
    }).raise_for_status()

    post("http://localhost:8000/server/join", headers={
        "servername": i,
        "username": i
    }).raise_for_status()

def server_join():
    for i in ["a", "b", "c"]:
        for i2 in ["a", "b", "c"]:
            post("http://localhost:8000/server/join", headers={
                "servername": i,
                "username": i2
            }).raise_for_status()

server_join()
