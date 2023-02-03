import pathlib

import requests


def main():
    root_dir = pathlib.Path(__file__).parent
    readme_path = root_dir.joinpath("README.md")
    assert readme_path.exists(), f"Path not found: {readme_path}"

    # TODO: Learn to use regex
    # pattern: "  * [name](url)"
    papers = dict()
    with open(readme_path, 'r') as reader:
        for line in reader.readlines():
            if not line.startswith("  * "):
                continue
            line = line.strip()
            [_, name_url] = line.split('[')
            [name, url] = name_url.split(']')
            url = url[1:-1]
            papers[name] = url

    references_dir = root_dir.joinpath("references")
    references_dir.mkdir(exist_ok=True)
    for name, url in papers.items():
        path = references_dir.joinpath(f"{name}.pdf")
        if not path.exists():
            with open(path, 'wb') as writer:
                writer.write(requests.get(url).content)

    return


if __name__ == "__main__":
    main()
    print("Success!")
