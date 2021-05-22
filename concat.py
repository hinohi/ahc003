import re
from pathlib import Path


def run(root: Path):
    for line in root.open():
        if m := re.match(r'mod ([^;]+);', line):
            mod = m.group(1)
            path = root.parent / f'{mod}.rs'
            print(f'mod {mod} {{')
            run(path)
            print('}')
        else:
            print(line, end='')


if __name__ == '__main__':
    run(Path('solver/src/main.rs'))
