import inspect

from icecube import icecube


def main():
    # list(map(print, filter(lambda name: "sum_as_string" in name[0] or "icecube" in name[0], inspect.getmembers(icecube))))
    list(map(print, inspect.getmembers(icecube.sum_as_string)))
    return


if __name__ == "__main__":
    main()
