import json
import os
import sys

INPUT_FILE = "invite_data.json"
GUILDS_FILE = "guilds.json"
TOP_N = 100


def load_known_guild_ids():
    if not os.path.exists(GUILDS_FILE):
        return set()

    with open(GUILDS_FILE, encoding="utf-8") as f:
        accounts = json.load(f)

    known_ids = set()
    for account in accounts:
        for guild in account.get("guilds") or []:
            guild_id = guild.get("id")
            if guild_id is not None:
                known_ids.add(guild_id)
    return known_ids


def main():
    with open(INPUT_FILE, encoding="utf-8") as f:
        data = json.load(f)

    known_guild_ids = load_known_guild_ids()

    GUILD_ID_MIN = 400_000_000_000_000_000

    best_by_guild = {}
    for entry in data:
        guild_id = entry.get("guild_id")
        count = entry.get("approximate_presence_count")
        guild_name = entry.get("guild_name")

        if guild_id is None or count is None or guild_id <= GUILD_ID_MIN:
            continue

        if guild_id in known_guild_ids:
            continue

        #name_lower = guild_name.lower()
        #if "front" not in name_lower: #and "nitro" not in name_lower:
        #    continue

        current = best_by_guild.get(guild_id)
        if current is None or count > current["approximate_presence_count"]:
            best_by_guild[guild_id] = entry

    top = sorted(
        best_by_guild.values(),
        key=lambda e: e["approximate_presence_count"],
        reverse=True,
    )[:TOP_N]

    for i, entry in enumerate(top, start=1):
        id_val = entry["guild_id"]
        name = entry.get("guild_name", "?")
        members = entry.get("approximate_member_count", 0)
        online = entry.get("approximate_presence_count", 0)
        code = entry.get("code", "?")
        print(f"{i:>3}. {name!r:<40} members={members:<10} online={online:<10} invite={code:<20} guild_id={id_val}")


if __name__ == "__main__":
    main()
