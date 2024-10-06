import json
import os
import requests

APP_ID = os.environ["DISCORD_APPLICATION_ID"]
BOT_TOKEN = os.environ["DISCORD_TOKEN"]

# see documentation
# https://discord.com/developers/docs/interactions/application-commands#endpoints
url = f'https://discord.com/api/v10/applications/{APP_ID}/commands'

json = [
  {
    'name': 'hello',
    'description': 'Hello world command',
    'options': []
  }
]

response = requests.put(
    url,
    headers={'Authorization': f'Bot {BOT_TOKEN}'},
    json=json
)

print(response.json()[0])
