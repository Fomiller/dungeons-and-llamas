import json
import os
import requests

APP_ID = os.environ["DISCORD_APPLICATION_ID"]
BOT_TOKEN = os.environ["DISCORD_TOKEN"]
CHANNEL_ID = "1293632547586052168"
INTERACTION = "1294442203418066986"
MSG_ID = "1318408585688125500"

# the interaction token has to be the token of the message_id that you want to edit
# store this token when a message that we know we will want to edit is handled and reference it later
INTERACTION_TOKEN = "aW50ZXJhY3Rpb246MTMxODQwODU4MTk4MDA5NDQ5NDo4ZW5QazBSZGdnc3JuV0FRS1RZN3FQMkV6SUJCbVV5SVc4WU9iOWlteDFFaVFMTjhqTWl2N2VzRTJqQ1VjNVFFb3JvSXl2THFrS25raDZKSTQ0aHdISDlQWGhPRVRsVHlxU2xEUXo3bXJqMWNHZk1wS0paTEpSbVZwVG5kUHhoWA"

# see documentation
# https://discord.com/developers/docs/interactions/application-commands#endpoints
url = f'https://discord.com/api/v10/webhooks/{APP_ID}/{INTERACTION_TOKEN}/messages/{MSG_ID}'
print(url)

json = {
    "content": "CAR",
}
response = requests.patch(
    url,
    headers={'Authorization': f'Bot {BOT_TOKEN}'},
    json=json
)

print(response.status_code)
print(response.json())
