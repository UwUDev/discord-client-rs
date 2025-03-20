def get_progress(path):
    content = None
    with open(path, 'r') as file:
        content = file.read()

    done = content.count("[x]")
    todo = content.count("[ ]")
    total = done + todo

    return f"{done}/{total} ({done/total*100:.2f}%)"

events = get_progress('discord_client_gateway/README.md')
print(f"Events progress: {events}")
endpoints = get_progress('discord_client_rest/README.md')
print(f"Endpoints progress: {endpoints}")