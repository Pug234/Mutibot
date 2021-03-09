import os
import threading
import atexit



def rustbot():
    os.system('cmd /k "cd rsbot & cargo run"')

def pybot():
    os.system('cmd /k "cd pybot & python -i main.py"')

def jsbot():
    os.system('cmd /k "cd jsbot & node main.js"')

def on_exit():
    os.system('cmd /k "cd C:/Users/corbi/OneDrive/Documents/rust/multibot"')



threading.Thread(target=rustbot).start()
threading.Thread(target=pybot).start()
threading.Thread(target=jsbot).start()

atexit.register(on_exit())
