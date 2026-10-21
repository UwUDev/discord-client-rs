#!/usr/bin/env python3
import json
import sys

TYPES = {"o", "N", "x", "u", "T", "F"}
# differ between the capture page and the about:blank iframe the challenge runs in.
KEEP_OLD = {"d.doctype", "d.currentScript", "d.all", "frameElement", "originAgentCluster", "Array"}
SKIP = {"constructor", "n.constructor", "d.constructor"}

capture = json.load(open(sys.argv[1]))
payload = json.load(open(sys.argv[2]))

old = {name: group for group, names in payload.items() for name in names}

groups = {}
for name, cls in capture.items():
    if name in SKIP:
        continue
    if name in old and (name in KEEP_OLD or old[name] not in TYPES):
        group = old[name]
    elif cls in TYPES:
        group = cls
    else:
        continue  # new string/number property: value depends on the page, not emulated
    groups.setdefault(group, []).append(name)

# keep the existing group order, then append groups that only exist in the capture
result = {g: groups.pop(g) for g in payload if g in groups}
result.update(groups)

added = sorted(n for g in result.values() for n in g if n not in old)
removed = sorted(n for n in old if n not in {n for g in result.values() for n in g})
print(f"added {len(added)}: {added}", file=sys.stderr)
print(f"removed {len(removed)}: {removed}", file=sys.stderr)

json.dump(result, open(sys.argv[2], "w"), separators=(",", ":"), ensure_ascii=False)
