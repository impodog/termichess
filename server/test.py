import requests

ADDR = "http://localhost:8080"
room = None
player = None

def login():
    global player, room

    if room is not None:
        print("You are already in a room")
        return
    
    room = input("Enter room: ")
    try:
        room = int(room)
    except ValueError:
        print("Invalid room")
        return
    response = requests.post(f"{ADDR}/chess/login", json={"room": room})
    print(response.json())
    player = response.json()["player"]
    

def query():
    response = requests.post(f"{ADDR}/chess/query", json={"room": room, "player": player})
    print(response)
    if response.status_code == 200:
        print(response.json())

def play():
    cmd = input("Enter command: ")
    response = requests.post(f"{ADDR}/chess/play", json={"room": room, "player": player, "cmd": cmd})
    print(response)
    if response.status_code == 200:
        print(response.json())
    
def logout():
    global room, player
    response = requests.post(f"{ADDR}/chess/logout", json={"room": room})
    print(response)
    if response.status_code == 200:
        print(response.json())
    room = player = None


def main():
    while True:
        method = input("Enter method(login/query/play/logout): ")
        match method:
            case "login":
                login()
            case "query":
                query()
            case "play":
                play()
            case "logout":
                logout()
            case _:
                print("Invalid method")
                
if __name__ == "__main__":
    main()