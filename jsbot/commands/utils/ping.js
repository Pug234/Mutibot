const { Command } = require('nukejs');
const Discord = require('discord.js');


module.exports = class extends Command {
    constructor(file) {
        super(file, {
            name: "jsping",
            runIn: ["text"]
        });
    }

    async run(message, args, client) {
      const Embed = new Discord.MessageEmbed()
      	.setColor('#361d9a')
      	.setTitle('Latency')
      	.addFields(
      		{ name: `Bot latency`, value: `${Date.now() - message.createdTimestamp} ms`, inline: true},
      		{ name: `API latency`, value: `${Math.round(client.ws.ping)} ms`, inline: true },
      	)

        message.channel.send(Embed);

    }
}
