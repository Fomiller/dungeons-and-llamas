import json
import os
import requests

APP_ID = os.environ["DISCORD_APPLICATION_ID"]
BOT_TOKEN = os.environ["DISCORD_TOKEN"]
CHANNEL_ID = "1293632547586052168"
INTERACTION = "1294442203418066986"
MSG_ID = "1294448188408991806"

# the interaction token has to be the token of the message_id that you want to edit
# store this token when a message that we know we will want to edit is handled and reference it later
INTERACTION_TOKEN = "aW50ZXJhY3Rpb246MTI5NDQ0ODE4NTU2NTI1MzY2MjpMRkluQmJ6Z2xISXdEV0c0aHlhSkdSbUpGbmw5a3c0OWxmR01pa0llcU1PTU5MZEpiUkxVMGJPWlNEWklxcm5BRk1OQnRMOElqd2dhcFNPZTNrYUlpS2w0cWVNOEFZUEl1cWY1M1BmdUFiRXV0Rm1Dcm44OFV1ZWk4TFJHbGd2NQ"

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
