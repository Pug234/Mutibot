const nukejs = require("nukejs")
require('dotenv').config({path:'../.env'});

let client = new nukejs.Client({});

const normalCommands = new nukejs.CommandLoader(client, {directory: "./commands", prefix: process.env.PREFIX , allowMention: true});

client.login(process.env.DISCORD_TOKEN);
