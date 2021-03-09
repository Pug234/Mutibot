import discord
from core import embeds

commands = discord.ext.commands

class util(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.command()
    async def pyping(self, ctx):
        await ctx.send(embed=discord.Embed(title='Latency', description=f'{round(self.bot.latency * 1000)} ms', color=discord.Color.blurple()))

def setup(bot):
    bot.add_cog(util(bot))
