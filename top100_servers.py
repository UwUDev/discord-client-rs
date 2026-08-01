import json
import sys

INPUT_FILE = "invite_data.json"
TOP_N = 100


def main():
    with open(INPUT_FILE, encoding="utf-8") as f:
        data = json.load(f)

    best_by_guild = {}
    for entry in data:
        guild_id = entry.get("guild_id")
        count = entry.get("approximate_member_count")
        if guild_id is None or count is None:
            continue
        current = best_by_guild.get(guild_id)
        if current is None or count > current["approximate_member_count"]:
            best_by_guild[guild_id] = entry

    top = sorted(
        best_by_guild.values(),
        key=lambda e: e["approximate_member_count"],
        reverse=True,
    )[:TOP_N]

    for i, entry in enumerate(top, start=1):
        name = entry.get("guild_name", "?")
        members = entry.get("approximate_member_count", 0)
        online = entry.get("approximate_presence_count", 0)
        code = entry.get("code", "?")
        print(f"{i:>3}. {name!r:<40} membres={members:<10} en_ligne={online:<10} invite={code}")


if __name__ == "__main__":
    main()
