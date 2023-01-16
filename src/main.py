import time
def main():
    now = time.time()
    for i in range(0,1_000_000):
        print(i)
    print(f'time elapsed is {time.time()-now}')