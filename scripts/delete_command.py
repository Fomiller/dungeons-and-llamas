import json
import os
import sys

import requests

APP_ID = os.environ["DISCORD_APPLICATION_ID"]
BOT_TOKEN = os.environ["DISCORD_TOKEN"]
COMMAND_ID = sys.argv[1]

# see documentation
# https://discord.com/developers/docs/interactions/application-commands#endpoints
url = f'https://discord.com/api/v10/applications/{APP_ID}/commands/{COMMAND_ID}'

response = requests.delete(
    url,
    headers={'Authorization': f'Bot {BOT_TOKEN}'},
)
if response.status_code == 204:
    print("Command deleted.")
else:
    print("Could not delete command.")
