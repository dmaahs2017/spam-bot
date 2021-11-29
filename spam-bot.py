import pyrammer
import discord

hs_model = pyrammer.hs_model_read_from_json("./enron_model.json")

CLIENTID = "914942454996811876"
PERMISSIONINT = "534723950656"

print("This invite link for your bot is: " +
      f"https://discordapp.com/oauth2/authorize?client_id={CLIENTID}&scope=bot&permissions={PERMISSIONINT}")


class SpamBotClient(discord.Client):
    async def on_message(self, message):

        if not message.author.bot:
            prob = hs_model.predict_on_text(message.content)
            print(f"{prob * 100}% chance of spam: '{message.content}'")

            if prob > .85:
                await message.reply(f"{prob * 100} chance of spam.")


bot = SpamBotClient()
token = open("token.txt", "r").read()
bot.run("OTE0OTQyNDU0OTk2ODExODc2.YaUYkA.cMLIC8J9VraNR9h7b9DQH6jDuYM")
