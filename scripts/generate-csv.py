#!/usr/bin/env python3

import json
import os
import sys

PASS = "\u2705"
SKIP = "\u26a0"
FAIL = "\u274c"

def main():
    if len(sys.argv) != 2:
        print("usage: {sys.argv[0]} RESULTS_DIRECTORY");
        exit(1)

    results = {}

    for fname in os.listdir(sys.argv[1]):
        fsname = os.path.splitext(fname)[0]

        fname = os.path.join(sys.argv[1], fname)
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

    fsnames = set()
    for vals in results.values():
        fsnames.update(vals.keys())
    fsnames = list(fsnames)
    fsnames.sort()

    print("test," + ",".join(fsnames))
    for test in sorted(results.keys()):
        row = [test]
        for fs in fsnames:
            row.append(results[test].get(fs, SKIP))
        print(",".join(row))



if __name__ == "__main__":
    main()
