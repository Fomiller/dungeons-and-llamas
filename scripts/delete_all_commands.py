import json
import os
import requests

APP_ID = os.environ["DISCORD_APPLICATION_ID"]
BOT_TOKEN = os.environ["DISCORD_TOKEN"]

# see documentation
# https://discord.com/developers/docs/interactions/application-commands#endpoints
url = f'https://discord.com/api/v10/applications/{APP_ID}/commands'

json = []

response = requests.put(
    url,
    headers={'Authorization': f'Bot {BOT_TOKEN}'},
    json=json
)
print(response.json())
