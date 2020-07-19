import time
from functools import wraps

from colorama import init, Fore

init(autoreset=True)


def timer(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print(Fore.GREEN + "function " + Fore.RED + f"`{func.__name__}` " + Fore.GREEN + "started")
        start_time = time.perf_counter_ns()
        result = func(*args, **kwargs)
        end_time = time.perf_counter_ns()
        print(Fore.GREEN + "function " + Fore.RED + f"`{func.__name__}` " + Fore.GREEN + "ended, costing time: " + 
              Fore.BLUE + f"{end_time-start_time} ns")

        return result

    return wrapper
