#!/usr/bin/env python3

import json
import os
import subprocess as sp
import sys

import yaml

PASS = "\u2705"
SKIP = "\u26a0"
FAIL = "\u274c"

def load_report_defs(fname):
    with open(fname) as handle:
        return yaml.load(handle, Loader=yaml.Loader)

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


def is_open_macro(tname):
    return tname.startswith("open_") and "_" in tname.rsplit("_", 1)[0]


def load_test_info(results):
    tinfo = {}

    for tname in results.keys():
        assert tname not in tinfo, f"Duplicate test name {tname}"
        if is_open_macro(tname):
            tinfo[tname] = load_macro_open(tname)
        else:
            tinfo[tname] = load_test_fn(tname)

    return tinfo


def load_macro_open(tname):
    #print(f"Loading open test: {tname}")
    cmd = f"rg --no-heading --line-number '{tname}:' src/tests"
    stdout = sp.check_output(cmd, shell=True).decode("utf-8")
    lines = stdout.splitlines()
    if len(lines) != 1:
        print(f"Error locating test info for {tname}")
        print("Found:")
        print(stdout)

    bits = lines[0].split(":", 2)
    assert len(bits) == 3, f"Bad open test line: {lines[0]}"
    fname = bits[0]
    lineno = bits[1]
    bits = bits[2].split(":", 1)
    assert len(bits) == 2, f"Bad open test data: {lines[0]}"
    bits = bits[1].strip().lstrip("(").rstrip("),").split(",", 3)
    assert len(bits) == 4, f"Bad test definition: {lines[0]}"
    perms = bits[0].strip().replace("libc::", "")
    opts = bits[1].strip().replace("libc::", "")
    if bits[2].strip() == "true":
        assert bits[3].strip() == "0"
        error = None
    else:
        error = bits[3].strip().replace("libc::", "")
    return {
        "type": "macro_open",
        "fname": fname,
        "lineno": lineno,
        "perms": perms,
        "opts": opts,
        "error": error
    }


def load_test_fn(tname):
    #print(f"Loading normal test: {tname}")
    cmd = f"rg --no-heading --line-number '((/// )|(fn )){tname}' src/tests"
    stdout = sp.check_output(cmd, shell=True).decode('utf-8')
    lines = list(sorted(set(stdout.splitlines())))
    if len(lines) != 2:
        print(f"Error locating test info for {tname}")
        print("Found:")
        print(stdout)
        exit(2)

    fname = None
    desc = None
    lineno = None
    for line in lines:
        bits = line.split(":", 2)
        assert len(bits) == 3, f"Invalid test info: {line}"
        fname = bits[0]
        if bits[2].startswith("///"):
            desc = bits[2].split(":", 1)[1].strip()
        else:
            lineno = bits[1]

    assert fname is not None, f"Invalid test info f{stdout}"
    assert desc is not None, f"Invalid test info f{stdout}"
    assert lineno is not None, f"Invalid test info f{stdout}"
    return {
        "type": "test_f",
        "fname": fname,
        "desc": desc,
        "lineno": lineno
    }


def generate_overview(defs, results):
    rows = []
    fsnames = set()
    for v in results.values():
        fsnames.update(v.keys())
    fsnames = list(sorted(fsnames))
    rows.append(["Topic", "Num. Tests"] + fsnames)
    rows.append(["-----"] + ["-----:"] * (1 + len(fsnames)))
    for topic in sorted(defs["topics"]):
        total = 0
        counts = {}
        for tname, values in results.items():
            if tname.rsplit("_", 1)[0] != topic:
                continue
            total += 1
            for (fsname, res) in values.items():
                counts.setdefault(fsname, 0)
                if res == PASS:
                    counts[fsname] += 1
        link = f"[{topic}](#{topic})"
        row = [link, str(total)]
        for fsname in fsnames:
            row.append(str(counts.get(fsname, 0)))
        rows.append(row)

    ret = ["## Summary Results", ""]
    for row in rows:
        row = "| " + " | ".join(row) + " |"
        ret.append(row)
    ret.append("")
    return "\n".join(ret)


def generate_topics(defs, results, info, sha):
    parts = []

    for topic in defs["topics"]:
        if topic.startswith("open") and "_" in topic:
            lines = generate_macro_open_topic(topic, defs, results, info, sha)
        else:
            lines = generate_test_fn_topic(topic, defs, results, info, sha)

        parts.append("\n".join(lines))

    return parts


def generate_macro_open_topic(topic, defs, results, info, sha):
    lines = ["", f"## {topic}", "[(top)](#summary-results)", ""]
    lines.append(defs["topics"][topic])
    lines.append("")
    lines.extend(generate_test_result_table(topic, results))
    lines.append("")

    tnames = []
    for tname in results.keys():
        if tname.rsplit("_", 1)[0] != topic:
            continue
        tnames.append(tname)
    tnames.sort()

    lines.append("")

    for tname in tnames:
        perms = info[tname]["perms"]
        opts = info[tname]["opts"]
        error = info[tname].get("error") or "Success"

        # Special case open_creat cause I'm lazy
        if topic == "open_creat":
            opts = "O_CREAT | " + opts

        link = make_source_link(defs, info, sha, tname)
        lines.append(f"### {tname}")
        lines.append(f"[(top)](#summary-results) [(table)](#{topic}) [(source)]({link})")
        lines.append("")
        lines.append("| Arg | Value |")
        lines.append("| ----- | ----- |")
        lines.append(f"| Permissions | {perms} |")
        lines.append(f"| Options | {opts} |")
        lines.append(f"| Result | {error} |")
        lines.append("")

    lines.append("")

    return lines


def generate_test_fn_topic(topic, defs, results, info, sha):
    lines = ["", f"## {topic}", "[(top)](#summary-results)", ""]
    lines.append(defs["topics"][topic])
    lines.append("")
    lines.extend(generate_test_result_table(topic, results))
    lines.append("")

    tnames = []
    for tname in results.keys():
        if tname.rsplit("_", 1)[0] != topic:
            continue
        tnames.append(tname)
    tnames.sort()

    for tname in tnames:
        link = make_source_link(defs, info, sha, tname)
        lines.append(f"### {tname}")
        lines.append(f"[(top)](#summary-results) [(table)](#{topic}) [(source)]({link})")
        lines.append("")
        lines.append(info[tname]["desc"])
        lines.append("")

    return lines

def generate_test_result_table(topic, results):
    fsnames = set()
    for v in results.values():
        fsnames.update(v.keys())
    fsnames = list(sorted(fsnames))

    tnames = []
    for tname in results:
        if tname.rsplit("_", 1)[0] != topic:
            continue
        tnames.append(tname)
    tnames.sort()

    table = []
    table.append(["Test"] + fsnames)
    table.append(["-----"] + [":-----:"] * len(fsnames))
    for tname in tnames:
        link = f"[{tname}](#{tname})"
        row = [link]
        for fsname in fsnames:
            row.append(results[tname].get(fsname, SKIP))
        table.append(row)

    ret = []
    for row in table:
        ret.append("| " + " | ".join(row) + " |")
    return ret


def make_source_link(defs, info, sha, tname):
    repo = defs["repo"]
    fname = info[tname]["fname"]
    lineno = info[tname]["lineno"]
    return f"{repo}/blob/{sha}/{fname}#L{lineno}"


def main():
    if len(sys.argv) != 3:
        print("usage: {sys.argv[0]} REPORT_DEFS RESULTS_DIRECTORY");
        exit(1)

    defs = load_report_defs(sys.argv[1])
    results = gather_results(sys.argv[2])
    info = load_test_info(results)

    sha = sp.check_output("git rev-parse --short HEAD", shell=True).decode("utf-8").strip()

    parts = [
        defs["prelude"],
        "\n",
        generate_overview(defs, results)
    ]
    parts.extend(generate_topics(defs, results, info, sha))

    print("".join(parts))

if __name__ == "__main__":
    main()
