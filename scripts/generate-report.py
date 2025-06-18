#!/usr/bin/env python3

import json
import os
import re
import subprocess as sp
import sys

PASS = "\u2705"
SKIP = "\u26a0"
FAIL = "\u274c"

DESC_RE = re.compile(r"^/// [a-z]_\d+:\s+(.*)")
DEFN_RE = re.compile(r"^fn [a-z]_\d+\(\)")

def load_report_defs(fname):
    return None

def gather_results(dname):
    results = {}

    for fname in os.listdir(dname):
        fsname = os.path.splitext(fname)[0]

        fname = os.path.join(dname, fname)
        if not os.path.isfile(fname):
           continue

        with open(fname) as handle:
            for line in handle:
                data = json.loads(line)
                if data["type"] != "test":
                    continue

                if data["event"] == "started":
                    continue

                name = data["name"].split("::")[-1]
                results.setdefault(name, {})

                if data["event"] == "ok":
                    results[name][fsname] = PASS
                elif data["event"] == "ignored":
                    raise RuntimeError("Test ignored")
                elif data["event"] == "failed":
                    results[name][fsname] = FAIL

    return results


def load_test_info(results):
    for tname in results.keys():
        if tname.startswith("open_") and "_" in tname.rsplit("_", 1)[0]:
            load_open(tname)
        else:
            load_normal(tname)


def load_open(tname):
    print(f"Loading open test: {tname}")
    cmd = f"rg --no-heading --line-number '{tname}:' src/tests"
    stdout = sp.check_output(cmd, shell=True)
    lines = stdout.splitlines()
    if len(lines) != 1:
        print(f"Error locating test info for {tname}")
        print("Found:")
        print(stdout.decode("utf-8"))


def load_normal(tname):
    print(f"Loading normal test: {tname}")
    cmd = f"rg --no-heading --line-number '((/// )|(fn )){tname}' src/tests"
    stdout = sp.check_output(cmd, shell=True)
    lines = list(sorted(set(stdout.splitlines())))
    if len(lines) != 2:
        print(f"Error locating test info for {tname}")
        print("Found:")
        print(stdout.decode("utf-8"))
        exit(2)


def main():
    if len(sys.argv) != 3:
        print("usage: {sys.argv[0]} REPORT_DEFS RESULTS_DIRECTORY");
        exit(1)

    report_defs = load_report_defs(sys.argv[1])
    results = gather_results(sys.argv[2])
    test_info = load_test_info(results)

if __name__ == "__main__":
    main()
