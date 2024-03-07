#!/usr/bin/env python3

# Requires:
# `pip install GitPython docker appdirs`
import argparse
import os
import sys
from cache import Cache
from appdirs import AppDirs

from project_instance import ProjectInstance


def common_entries(*dcts):
    if not dcts:
        return
    for i in set(dcts[0]).intersection(*dcts[1:]):
        yield (i,) + tuple(d[i] for d in dcts)


def list_dirs(path):
    entries = map(lambda p: os.path.join(path, p), os.listdir(path))
    return filter(os.path.isdir, entries)


def report(main, this_branch):
    def diff(old, new):
        diff = (new - old) / old

        return "{0:+.0%}".format(diff)

    header = """# Contract size report

Sizes are given in bytes.

| contract | main | this branch | difference |
| - | - | - | - |"""

    combined = [
        (name, main, branch, diff(main, branch))
        for name, main, branch in common_entries(main, this_branch)
    ]
    combined.sort(key=lambda el: el[0])
    rows = [f"| {name} | {old} | {new} | {diff} |" for name, old, new, diff in combined]

    return "\n".join([header, *rows])


def main():
    parser = argparse.ArgumentParser(
        prog="compare_sizes",
        description="compare example contract sizes between current branch and main",
    )
    parser.add_argument("-c", "--cargo-cache-dir")
    args = parser.parse_args()

    default_cache_dir = os.path.join(
        AppDirs("unc_sdk_dev_cache", "unc").user_data_dir,
        "contract_build",
    )
    cache_dir = args.cargo_cache_dir if args.cargo_cache_dir else default_cache_dir
    cache = Cache(cache_dir)

    this_file = os.path.abspath(os.path.realpath(__file__))
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(this_file)))

    cur_branch = ProjectInstance(project_root)

    with cur_branch.branch("main") as main:
        cur_sizes = cur_branch.sizes(cache)
        main_sizes = main.sizes(cache)

        print(report(main_sizes, cur_sizes))


if __name__ == "__main__":
    main()
